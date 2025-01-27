import re
import os
import json
import pre_process
import chardet
import sys
from collections import defaultdict

def find_file_in_directory(root_dir, filename):
    for root, dirs, files in os.walk(root_dir):
        if filename in files:
            return os.path.abspath(os.path.join(root, filename))
    return None

def remove_comments(code):
    # 去除单行注释
    code_no_single_line_comments = re.sub(r'//.*', '', code)
    
    # 去除多行注释
    code_no_comments = re.sub(r'/\*.*?\*/', '', code_no_single_line_comments, flags=re.DOTALL)

    code_no_conditions = re.sub(r'#(if|ifdef|ifndef|else|endif)\b.*?(\n|$)', '', code_no_comments, flags=re.DOTALL)
    
    return code_no_conditions

def parse_enum_body(enum_body):
    """
    解析枚举主体字符串，支持检测十进制和十六进制值。
    返回一个字典，包含枚举成员名、值、进制和显式标记。
    """
    # 正则表达式匹配枚举成员
    member_pattern = re.compile(r'(\w+)\s*(?:=\s*(0x[0-9a-fA-F]+|\d+))?')
    members = member_pattern.findall(enum_body)
    
    enum_members = {}
    last_value = None  # 跟踪上一个成员的值
    
    for member, value in members:
        if value:  # 如果有显式值
            if value.startswith('0x') or value.startswith('0X'):  # 检测是否为十六进制
                value = int(value, 16)  # 转换为整数
                is_hex = True
            else:
                value = int(value)  # 转换为整数
                is_hex = False
            is_explicit = True  # 显式指定值
        else:  # 如果没有显式值，自动递增
            value = (last_value + 1) if last_value is not None else 0
            is_hex = False
            is_explicit = False
        
        last_value = value
        enum_members[member] = (value, is_hex, is_explicit)
    
    return enum_members

def extract_enum_info_c(filename):
    with open(filename, 'rb') as file:
        raw_data = file.read()
        result = chardet.detect(raw_data)
        encoding = result['encoding']

    # 使用检测到的编码打开文件
    with open(filename, 'r', encoding=encoding) as file:
        code = file.read()
    
    code = remove_comments(code)
    
    enum_info = []

    # 匹配枚举的定义（包括匿名枚举）
    enum_pattern = re.compile(r'enum\s*(\w*)\s*\{(.*?)\}\s*;', re.DOTALL)

    # 匹配枚举类型别名的定义
    typedef_pattern = re.compile(r'typedef\s+enum\s*\{(.*?)\}\s*(\w+)\s*;', re.DOTALL)

    for match in typedef_pattern.finditer(code):
        # 提取枚举内容和类型别名名称
        enum_body = match.group(1).strip()     # 枚举体
        enum_name = match.group(2).strip()     # 类型别名名称
        start_idx = match.start()              # 起始位置
        end_idx = match.end()                  # 结束位置
        enum_members = {}
        
        enum_members = parse_enum_body(enum_body)

        # 保存结果
        enum_info.append({
            'enum_name': enum_name,
            'members': enum_members,
            'start_pos': start_idx,
            'end_pos': end_idx
        })

    typedef_enum_pattern = r"typedef\s+enum\s*\{.*?\}\s+[A-Za-z_][A-Za-z0-9_]*\s*;"
    code_without_typedef = re.sub(typedef_enum_pattern, "", code, flags=re.DOTALL)
    
    for match in enum_pattern.finditer(code_without_typedef):
        enum_body = match.group(2).strip()     # 枚举体
        enum_name = match.group(1).strip()     # 枚举名称
        start_idx = match.start()              # 起始位置
        end_idx = match.end()                  # 结束位置
        enum_members = {}
        
        enum_members = parse_enum_body(enum_body)
        
        enum_info.append({
            'enum_name': enum_name,
            'members': enum_members,
            'start_pos': start_idx,
            'end_pos': end_idx
        })

    
    # 查找枚举的使用位置
    # usage_pattern = re.compile(r'(\w+)\s*=\s*(\w+)\s*;\s*|\b(\w+)\b')
    # enum_usage = []
    
    # for match in usage_pattern.finditer(code):
    #     usage_info = match.group()
    #     enum_usage.append(usage_info)
    
    return enum_info

def extract_enum_info_rust(filename):
    """
    提取给定 Rust 文件中所有符合特定模式的枚举信息。
    
    :param filepath: `.rs` 文件的路径
    :return: 解析得到的枚举信息，格式为一个字典列表，每个字典包含枚举名称、成员名称和值
    """
    # 定义枚举类型的正则表达式，匹配 "pub type {enum_name} = libc::c_uint;"
    enum_type_pattern = re.compile(r'pub type\s+(\w+)\s*=\s*libc::c_uint;')
    
    # 定义枚举成员的正则表达式，匹配 "pub const {member_name}: {enum_name} = {member_value};"
    enum_member_pattern = re.compile(r'pub const\s+(\w+)\s*:\s*(\w+)\s*=\s*(\d+);')

    # 存储枚举信息
    enum_info_list = []
    
    # 打开文件读取内容
    with open(filename, 'r', encoding='utf-8') as file:
        code = file.read()

    code = remove_comments(code)
    
    # 查找所有匹配的枚举类型定义
    enum_types = enum_type_pattern.findall(code)
    enum_members = enum_member_pattern.findall(code)

    # 遍历每个枚举类型
    for enum_name in enum_types:      

        # 过滤出符合当前枚举名称的成员
        filtered_members = {}
        last_member_end_pos = -1  # 记录最后一个成员定义的结束位置

        for member_name, member_enum, member_value in enum_members:
            if member_enum == enum_name:
                filtered_members[member_name] = (int(member_value), False, True)
                member_code = f"pub const {member_name}: {enum_name} = {member_value};"
                last_member_end_pos = code.find(member_code) + len(member_code)
        
        if not filtered_members:
            continue
        
        # 记录每个枚举的位置信息
        start_pos = code.find(f'pub type {enum_name} = libc::c_uint;')
        end_pos = last_member_end_pos

        enum_info_list.append({
            'enum_name': enum_name,
            'start_pos': start_pos,
            'end_pos': end_pos,
            'members': filtered_members
        })

    return enum_info_list

def make_enum_code_c(proj_path):
    if not os.path.exists('enum_code_c'):
        os.makedirs('enum_code_c')

    prog = os.path.basename(proj_path)
    path_to_candidate = pre_process.get_candidate_path(prog)
    enum_json_file = path_to_candidate + '/enum.json'
    with open(enum_json_file, 'r', encoding='utf-8') as f:
        enums = json.load(f)
        
    result = {}

    for filename, enum_names in enums.items():
        filepath = find_file_in_directory(proj_path, filename)
        res = extract_enum_info_c(filepath)
        if len(res) > 0:
            result[filepath] = extract_enum_info_c(filepath)

    if not os.path.exists('enum_code_c/' + prog):
        os.makedirs('enum_code_c/' + prog)

    with open('enum_code_c/' + prog + '/enum.json', 'w') as json_file:
        json.dump(result, json_file)

def make_enum_code_rust(proj_path):
    if not os.path.exists('enum_code_rust'):
        os.makedirs('enum_code_rust')

    prog = os.path.basename(proj_path)
    path_to_candidate = 'candidate_rust/' + prog
    candidate_json_file = path_to_candidate + '/enum.json'
    with open(candidate_json_file, 'r', encoding='utf-8') as f:
        candidates = json.load(f)
        
    result = {}

    for candidate_path in candidates:
        file_enums = extract_enum_info_rust(candidate_path)
        if len(file_enums) > 0:
            result[os.path.abspath(candidate_path)] = file_enums

    if not os.path.exists('enum_code_rust/' + prog):
        os.makedirs('enum_code_rust/' + prog)

    with open('enum_code_rust/' + prog + '/enum.json', 'w') as json_file:
        json.dump(result, json_file)


def check_enum(enum_name, enum_members, c_enums):
    # 如果枚举名以 'C2RustUnnamed' 为前缀，根据成员名列表进行查找
    if enum_name.startswith('C2RustUnnamed'):
        for enum in c_enums:
            # 如果枚举成员列表一致，返回对应枚举
            if enum['members'].keys() == enum_members.keys():
                return enum
    else:
        # 如果枚举名不以 'C2RustUnnamed' 为前缀，则直接比较枚举名
        for enum in c_enums:
            if enum['enum_name'] == enum_name:
                return enum

    # 如果没有找到匹配的枚举，返回 None
    return None

def find_enum_definition(enum_name, enum_members, c_file_path, file_references, c_enums):
    """
    根据枚举名称和 rs 文件路径，递归查找 C 文件中的同名枚举定义。
    如果在当前 C 文件中未找到，则查找引用的 C 文件。
    """
    if c_file_path not in c_enums and c_file_path not in file_references:
        c_file_path = c_file_path.replace('_', '-')

    if c_file_path in c_enums:
        # 在 C 文件中查找枚举定义
        result = check_enum(enum_name, enum_members, c_enums[c_file_path])
        if result:
            # print(result)
            return result

    if c_file_path in file_references:
        # 如果没有找到同名 C 文件，查找其引用的文件
        for referenced_file in file_references[c_file_path]:
            if referenced_file in c_enums:
                result = check_enum(enum_name, enum_members, c_enums[referenced_file])
                if result:
                    # print(result)
                    return result

    # 如果找不到该枚举定义，返回 None
    return None

def transfer_enum_rust(proj_path):

    prog = os.path.basename(proj_path)
    path_to_enum_rust_json_file = 'enum_code_rust/' + prog + '/enum.json'
    with open(path_to_enum_rust_json_file, 'r', encoding='utf-8') as f:
        rust_enums = json.load(f)

    path_to_enum_c_json_file = 'enum_code_c/' + prog + '/enum.json'
    with open(path_to_enum_c_json_file, 'r', encoding='utf-8') as f:
        c_enums = json.load(f)
        
    path_to_c_include_list = 'c_include_list/' + prog + '/c_include_list.json'
    with open(path_to_c_include_list, 'r', encoding='utf-8') as f:
        c_include_list = json.load(f)

    for file_path, enums in rust_enums.items():
        for enum in enums:
            enum_name = enum['enum_name']
            enum_members = enum['members']
            c_file_path = file_path.replace('.rs', '.c')
            c_enum = find_enum_definition(enum_name, enum_members, c_file_path, c_include_list, c_enums)
            if c_enum:
                if c_enum['enum_name']:
                    enum['enum_name'] = c_enum['enum_name']
                members = enum['members']
                c_members = c_enum['members']
                tmp_members = {}
                for member_name in c_members.keys():
                    if member_name in members:
                        tmp_members[member_name] = c_members[member_name]
                        tmp_members[member_name][0] = members[member_name][0]
                enum['members'] = tmp_members
        
    with open('enum_code_rust/' + prog + '/enum.json', 'w') as json_file:
        json.dump(rust_enums, json_file)


def convert_value(value, is_hex):
    """根据给定的进制将值转换为字符串"""
    if is_hex:
        return f"0x{value:X}"
    else:
        return str(value)  # 其他进制（默认为十进制）
    
def generate_enum_code(enum_name, members):
    """根据枚举名称和成员生成替换后的代码"""
    enum_code = f"#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]\n"
    enum_code += f"#[repr(C)]\n"
    enum_code += f"pub enum {enum_name} {{\n"
    
    for member, (value, is_hex, is_explict) in members.items():
        if is_explict:
            value_str = convert_value(value, is_hex)  # 根据进制转换值
            enum_code += f"    {member} = {value_str},\n"  # 添加枚举成员和对应的值
        else:
            enum_code += f"    {member},\n"
    
    enum_code += "}  // end of enum\n"

    enum_code += f"impl {enum_name} {{\n"

    enum_code += f"    fn to_libc_c_uint(self) -> libc::c_uint {{\n"
    enum_code += f"        match self {{\n"
    for member, (value, is_hex, is_explict) in members.items():
        value_str = convert_value(value, is_hex)
        enum_code += f"            {enum_name}::{member} => {value_str},\n"
    enum_code += f"        }}\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"
    return enum_code

def get_indentation_at_position(content, position):
    """获取给定位置所在行的缩进量"""
    # 获取该位置所在行
    line_start = content.rfind("\n", 0, position) + 1
    line_end = content.find("\n", line_start)
    line = content[line_start:line_end]
    
    # 计算缩进：前导空格或制表符数量
    indentation = len(line) - len(line.lstrip())
    return indentation

def apply_indentation(line, indentation):
    """根据指定的缩进量应用缩进"""
    return ' ' * indentation + line

def process_enum_in_file(file_path, enums_info):
    """处理指定文件中的枚举并替换为新的格式"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 调整枚举位置的偏移量（初始为0）
    offset = 0
    
    # 遍历枚举信息，替换文件中的枚举部分
    for enum_info in enums_info:
        enum_name = enum_info['enum_name']
        start_pos = enum_info['start_pos'] + offset
        end_pos = enum_info['end_pos'] + offset
        members = enum_info['members']

        # 获取当前枚举起始位置所在行的缩进
        indentation = get_indentation_at_position(content, start_pos)
        
        # 生成新的枚举代码块
        new_enum_code = generate_enum_code(enum_name, members)

         # 按照缩进调整新枚举代码的每一行
        new_enum_code_lines = new_enum_code.splitlines()
        indented_enum_code = "\n".join([apply_indentation(line, indentation) for line in new_enum_code_lines]) + "\n"
        
        # 替换文件中的旧枚举代码块
        content = content[:start_pos] + indented_enum_code + content[end_pos:]

        # 计算替换后新的内容长度差异，更新偏移量
        old_enum_length = end_pos - start_pos
        new_enum_length = len(indented_enum_code)
        offset += (new_enum_length - old_enum_length)  # 更新偏移量
    
    return content

def process_json_file(json_file_path, output_dir):
    """处理给定的 JSON 文件，并生成新的 Rust 文件"""
    # 读取 JSON 文件
    with open(json_file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    for rs_file_path, enums_info in data.items():
        if not os.path.exists(rs_file_path):
            print(f"File not found: {rs_file_path}")
            continue
        
        # 处理枚举并替换文本
        modified_content = process_enum_in_file(rs_file_path, enums_info)
        
        # 确保输出目录存在
        os.makedirs(output_dir, exist_ok=True)
        
        # 生成新的文件路径
        output_file_path = os.path.join(output_dir, os.path.basename(rs_file_path))
        
        # 将修改后的内容写入新文件
        with open(output_file_path, 'w', encoding='utf-8') as f:
            f.write(modified_content)
        

def refactoring(proj_path):
    if not os.path.exists('output'):
        os.makedirs('output')
    
    make_enum_code_c(proj_path)
    make_enum_code_rust(proj_path)
    transfer_enum_rust(proj_path)

    prog = os.path.basename(proj_path)
    path_to_json = 'enum_code_rust/' + prog + '/enum.json'
    path_to_output = 'output/' + prog
    process_json_file(path_to_json, path_to_output)

# make_enum_code_c()
# make_enum_code_rust()
# transfer_enum_rust()
# refactoring()

if __name__ == "__main__":
    if len(sys.argv) > 1:
        print("usage: refactoring program path or default path: ../c_prog/")
    if len(sys.argv) == 1:
        proj_path = sys.argv[0]
        refactoring(proj_path)

    else:
        for prog in pre_process.prog_list:
            proj_path = pre_process.get_prog_path(prog)
            refactoring(proj_path)