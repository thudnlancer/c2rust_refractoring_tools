import re
import os
import json
import pre_process
import chardet
from collections import defaultdict

def find_file_in_directory(root_dir, filename):
    for root, dirs, files in os.walk(root_dir):
        if filename in files:
            return os.path.join(root, filename)
    return None

def remove_comments(code):
    # 去除单行注释
    code_no_single_line_comments = re.sub(r'//.*', '', code)
    
    # 去除多行注释
    code_no_comments = re.sub(r'/\*.*?\*/', '', code_no_single_line_comments, flags=re.DOTALL)

    code_no_conditions = re.sub(r'#(if|ifdef|ifndef|else|endif)\b.*?(\n|$)', '', code_no_comments, flags=re.DOTALL)
    
    return code_no_conditions

def extract_enum_info_c(filename):
    with open(filename, 'rb') as file:
        raw_data = file.read()
        result = chardet.detect(raw_data)
        encoding = result['encoding']

    # 使用检测到的编码打开文件
    with open(filename, 'r', encoding=encoding) as file:
        code = file.read()
    
    code = remove_comments(code)
    
    # 匹配枚举的定义（包括匿名枚举）
    enum_pattern = re.compile(r'\b(enum)\s*(\w*)\s*{(.*?)}', re.DOTALL)
    
    enum_info = []
    
    for match in enum_pattern.finditer(code):
        enum_body = match.group(3).strip()     # 枚举体
        enum_name = match.group(2).strip()     # 枚举名称
        start_idx = match.start()              # 起始位置
        end_idx = match.end()                  # 结束位置
        enum_members = {}
        
        # 提取枚举成员和它们的值
        member_pattern = re.compile(r'(\w+)\s*(?:=\s*(\d+))?')
        members = member_pattern.findall(enum_body)
        
        last_value = -1
        for member, value in members:
            if value:
                value = int(value)
            else:
                value = last_value + 1
            last_value = value
            enum_members[member] = value
        
        enum_info.append({
            'enum_name': enum_name,
            'members': enum_members,
            'start_pos': start_idx,
            'end_pos': end_idx
        })

    # 匹配枚举类型别名的定义
    typedef_pattern = re.compile(r'typedef\s+enum\s*\{(.*?)\}\s*(\w+)')

    for match in typedef_pattern.finditer(code):
        # 提取枚举内容和类型别名名称
        enum_body = match.group(1).strip()     # 枚举体
        enum_name = match.group(2).strip()     # 类型别名名称
        start_idx = match.start()              # 起始位置
        end_idx = match.end()                  # 结束位置
        enum_members = {}
        
        # 提取枚举成员和它们的值
        member_pattern = re.compile(r'(\w+)\s*(?:=\s*(\d+))?')
        members = member_pattern.findall(enum_body)
        
        last_value = -1
        for member, value in members:
            if value:
                value = int(value)
            else:
                value = last_value + 1
            last_value = value
            enum_members[member] = value

        # 保存结果
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
                filtered_members[member_name] = (int(member_value), 10)
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

def make_enum_code_c():
    if not os.path.exists('enum_code_c'):
        os.makedirs('enum_code_c')
    for prog in pre_process.prog_list:
        path_to_candidate = pre_process.get_candidate_path(prog)
        path_to_prog = pre_process.get_prog_path(prog)
        enum_json_file = path_to_candidate + '/enum.json'
        with open(enum_json_file, 'r', encoding='utf-8') as f:
            enums = json.load(f)
        
        result = {}

        for filename, enum_names in enums.items():
            filepath = find_file_in_directory(path_to_prog, filename)
            result[filename] = extract_enum_info_c(filepath)

        if not os.path.exists('enum_code_c/' + prog):
            os.makedirs('enum_code_c/' + prog)

        with open('enum_code_c/' + prog + '/enum.json', 'w') as json_file:
            json.dump(result, json_file)

def make_enum_code_rust():
    if not os.path.exists('enum_code_rust'):
        os.makedirs('enum_code_rust')
    for prog in pre_process.prog_list:
        path_to_candidate = 'candidate_rust/' + prog
        candidate_json_file = path_to_candidate + '/enum.json'
        with open(candidate_json_file, 'r', encoding='utf-8') as f:
            candidates = json.load(f)
        
        result = {}

        for candidate_path in candidates:
            file_enums = extract_enum_info_rust(candidate_path)
            result[candidate_path] = file_enums

        if not os.path.exists('enum_code_rust/' + prog):
            os.makedirs('enum_code_rust/' + prog)

        with open('enum_code_rust/' + prog + '/enum.json', 'w') as json_file:
            json.dump(result, json_file)

def convert_value(value, base):
    """根据给定的进制将值转换为字符串"""
    if base == 16:
        return hex(value)  # 转换为十六进制
    else:
        return str(value)  # 其他进制（默认为十进制）
    
def generate_enum_code(enum_name, members):
    """根据枚举名称和成员生成替换后的代码"""
    enum_code = f"#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]\n"
    enum_code += f"#[repr(C)]\n"
    enum_code += f"pub enum {enum_name} {{\n"
    
    for member, (value, base) in members.items():
        value_str = convert_value(value, base)  # 根据进制转换值
        enum_code += f"    {member} = {value_str},\n"  # 添加枚举成员和对应的值
    
    enum_code += "}  // end of enum\n"
    return enum_code

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
        
        # 生成新的枚举代码块
        new_enum_code = generate_enum_code(enum_name, members)
        
        # 替换文件中的旧枚举代码块
        content = content[:start_pos] + new_enum_code + content[end_pos:]

        # 计算替换后新的内容长度差异，更新偏移量
        old_enum_length = end_pos - start_pos
        new_enum_length = len(new_enum_code)
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
        
        print(f"Processed file saved to: {output_file_path}")

def refactoring():
    if not os.path.exists('output'):
        os.makedirs('output')
    for prog in pre_process.prog_list:
        path_to_json = 'enum_code_rust/' + prog + '/enum.json'
        path_to_output = 'output/' + prog
        process_json_file(path_to_json, path_to_output)

# make_enum_code_c()
# make_enum_code_rust()
refactoring()