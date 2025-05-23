import re
import os
import json
import pre_process
import chardet
import sys
import subprocess

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
        first_member_end_pos = -1  # 记录最后一个成员定义的结束位置
        first_member_start_pos = -1  # 记录第一个成员定义的开始位置
        flag = False # 记录是否包含重复值成员，如果有则跳过处理

        for member_name, member_enum, member_value in enum_members:
            if member_enum == enum_name:
                if int(member_value) in {int(v) for v, _, _ in filtered_members.values()}:
                    flag = True
                    break
                filtered_members[member_name] = (int(member_value), False, True)
                member_code = f"pub const {member_name}: {enum_name} = {member_value};"
                if first_member_start_pos == -1:
                    first_member_start_pos = code.find(member_code)
                    first_member_end_pos = code.find(member_code) + len(member_code)
        if not filtered_members:
            continue

        # 如果包含重复值成员，则跳过处理
        if flag:
            continue
        
        # 记录每个枚举的位置信息
        if first_member_start_pos < code.find(f'pub type {enum_name} = libc::c_uint;'):
            start_pos = first_member_start_pos
            end_pos = first_member_end_pos
        else:
            start_pos = code.find(f'pub type {enum_name} = libc::c_uint;')
            end_pos = start_pos + len(f'pub type {enum_name} = libc::c_uint;')

        enum_info_list.append({
            'enum_name': enum_name,
            'start_pos': start_pos,
            'end_pos': end_pos,
            'members': filtered_members
        })

    # 根据 start_pos 对 enum_info_list 进行升序排序
    enum_info_list_sorted = sorted(enum_info_list, key=lambda x: x['start_pos'])

    return enum_info_list_sorted

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
    
    enum_code += f"}}\n"
    enum_code += f"impl {enum_name} {{\n"

    enum_code += f"    fn to_libc_c_uint(self) -> libc::c_uint {{\n"
    enum_code += f"        match self {{\n"
    for member, (value, is_hex, is_explict) in members.items():
        value_str = convert_value(value, is_hex)
        enum_code += f"            {enum_name}::{member} => {value_str},\n"
    enum_code += f"        }}\n"
    enum_code += f"    }}\n"
    enum_code += f"    fn from_libc_c_uint(value: libc::c_uint) -> {enum_name} {{\n"
    enum_code += f"        match value {{\n"
    for member, (value, is_hex, is_explict) in members.items():
        value_str = convert_value(value, is_hex)
        enum_code += f"            {value_str} => {enum_name}::{member},\n"
    enum_code += f"            _ => panic!(\"Invalid value for {enum_name}: {{}}\", value),\n"
    enum_code += f"        }}\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl AddAssign<u32> for {enum_name} {{\n"
    enum_code += f"    fn add_assign(&mut self, rhs: u32) {{\n"
    enum_code += f"        *self = {enum_name}::from_libc_c_uint(self.to_libc_c_uint() + rhs);\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl SubAssign<u32> for {enum_name} {{\n"
    enum_code += f"    fn sub_assign(&mut self, rhs: u32) {{\n"
    enum_code += f"        *self = {enum_name}::from_libc_c_uint(self.to_libc_c_uint() - rhs);\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl MulAssign<u32> for {enum_name} {{\n"
    enum_code += f"    fn mul_assign(&mut self, rhs: u32) {{\n"
    enum_code += f"        *self = {enum_name}::from_libc_c_uint(self.to_libc_c_uint() * rhs);\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl DivAssign<u32> for {enum_name} {{\n"
    enum_code += f"    fn div_assign(&mut self, rhs: u32) {{\n"
    enum_code += f"        *self = {enum_name}::from_libc_c_uint(self.to_libc_c_uint() / rhs);\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl RemAssign<u32> for {enum_name} {{\n"
    enum_code += f"    fn rem_assign(&mut self, rhs: u32) {{\n"
    enum_code += f"        *self = {enum_name}::from_libc_c_uint(self.to_libc_c_uint() % rhs);\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl Add<u32> for {enum_name} {{\n"
    enum_code += f"    type Output = {enum_name};\n"
    enum_code += f"    fn add(self, rhs: u32) -> {enum_name} {{\n"
    enum_code += f"        {enum_name}::from_libc_c_uint(self.to_libc_c_uint() + rhs)\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl Sub<u32> for {enum_name} {{\n"
    enum_code += f"    type Output = {enum_name};\n"
    enum_code += f"    fn sub(self, rhs: u32) -> {enum_name} {{\n"
    enum_code += f"        {enum_name}::from_libc_c_uint(self.to_libc_c_uint() - rhs)\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl Mul<u32> for {enum_name} {{\n"
    enum_code += f"    type Output = {enum_name};\n"
    enum_code += f"    fn mul(self, rhs: u32) -> {enum_name} {{\n"
    enum_code += f"        {enum_name}::from_libc_c_uint(self.to_libc_c_uint() * rhs)\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl Div<u32> for {enum_name} {{\n"
    enum_code += f"    type Output = {enum_name};\n"
    enum_code += f"    fn div(self, rhs: u32) -> {enum_name} {{\n"
    enum_code += f"        {enum_name}::from_libc_c_uint(self.to_libc_c_uint() / rhs)\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"

    enum_code += f"impl Rem<u32> for {enum_name} {{\n"
    enum_code += f"    type Output = {enum_name};\n"
    enum_code += f"    fn rem(self, rhs: u32) -> {enum_name} {{\n"
    enum_code += f"        {enum_name}::from_libc_c_uint(self.to_libc_c_uint() % rhs)\n"
    enum_code += f"    }}\n"
    enum_code += f"}}\n"



    # enum_code += "// end of enum replacement\n"
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

    content = clean_enum_definition(content, enums_info)

    content = process_enum_use(content, enums_info)
    
    return insert_use_statement(content)

def insert_use_statement(code):
    
    multi_line_attr = re.compile(
        r"(?s)(^#!\[\s*.*?\])"
    )

    # 单行 inner attribute
    single_line_attr = re.compile(
        r"(?m)^#!\[[^\n]*\]"
    )

    # 找到所有匹配
    multi_matches = list(multi_line_attr.finditer(code))
    single_matches = list(single_line_attr.finditer(code))

    # 获取所有匹配中末尾最大的
    all_matches = multi_matches + single_matches
    if all_matches:
        insert_pos = max(m.end() for m in all_matches)
    else:
        insert_pos = 0

    # 插入内容
    insertion = 'use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};\n'
    modified_code = code[:insert_pos] + '\n' + insertion + code[insert_pos:]

    return modified_code

def clean_enum_definition(content, enums_info):
    """清理枚举定义中的多余内容"""
    for enum_info in enums_info:

        # 匹配枚举定义的正则表达式
        pattern = r"pub type\s+" + re.escape(enum_info['enum_name']) + r"\s*=\s*libc::c_uint;"
        match = re.search(pattern, content)
        if match:
            # 提取枚举定义的缩进
            indentation = get_indentation_at_position(content, match.start())

            # 删除枚举定义
            enum_definition = match.group()
            enum_definition_lines = enum_definition.splitlines()
            cleaned_enum_definition = "\n".join([apply_indentation(line, indentation) for line in enum_definition_lines]) + "\n"
            content = content.replace(cleaned_enum_definition, "")

        # 匹配枚举成员定义的正则表达式
        member_pattern = r"pub const\s+(\w+)\s*:\s*" + re.escape(enum_info['enum_name']) + r"\s*=\s*[^;]+;"
        member_matches = list(re.finditer(member_pattern, content))

        if member_matches:

            for match in reversed(member_matches):  # 从后往前删除，避免位置偏移
                # 提取成员定义的缩进
                indentation = get_indentation_at_position(content, match.start())

                # 删除成员定义
                member_definition = match.group()
                member_definition_lines = member_definition.splitlines()
                cleaned_member_definition = "\n".join([apply_indentation(line, indentation) for line in member_definition_lines]) + "\n"
                content = content.replace(cleaned_member_definition, "")

    return content

def process_enum_use(content, enums_info):
    """处理内容中的枚举使用并替换内容"""
    for enum_info in enums_info:
        enum_name = enum_info['enum_name']
        members = enum_info['members']
        value_names = extract_keywords(content, enum_name)
        # 替换变量形式
        for value_name in value_names:
            content = process_value_switch(content, value_name, enum_name, members)
            content = process_value_cast(content, value_name)

        content = process_value_definition(content, members, enum_name)
        content = process_enum_cast(content, enum_name)

    return content

def process_enum_cast(content, enum_name):
    
    result = subprocess.run(
        ["./rust_code_analyzer/target/release/rust_code_analyzer", "--enum-name", enum_name],
        input=content,
        capture_output=True,
        text=True
    )

    if result.returncode == 0:
        return result.stdout.strip()
    else:
        return content

def process_value_switch(content, value_name, enum_name, members):
    # 构建值到成员名的反向映射字典（考虑十进制和十六进制）
    value_to_member = {}
    for member, (value, is_hex, *_) in members.items():
        # 记录十进制和可能的十六进制形式
        value_to_member[str(value)] = member
        if is_hex:
            value_to_member[f"0x{value:x}"] = member
            value_to_member[f"0x{value:X}"] = member  # 大写形式

    # 匹配整个 match 语句块
    pattern = (
        r"(match\s+" + re.escape(value_name) + r"\s*\{)"  # 匹配 match 头部
        r"([^}]*(?:\{[^{}]*\}[^}]*)*)"  # 匹配 match 主体（支持嵌套）
        r"(\})"  # 匹配右花括号
    )
    match = re.search(pattern, content, re.DOTALL)
    if not match:
        return content  # 未找到 match 块

    # 提取原始 match 头和尾
    match_head = match.group(1)
    original_body = match.group(2)
    match_tail = match.group(3)

    # 分割每个分支（处理可能存在的注释和格式）
    branch_pattern = re.compile(
        r"(\s*)(.*?)\s*=>\s*(.*?)(,?)(\s*(//.*?)?)$", 
        re.DOTALL | re.MULTILINE
    )
    new_branches = []
    for branch in re.split(r"(?<=},\n)", original_body):  # 按分支分割
        branch = branch.strip()
        if not branch:
            continue

        # 解析分支结构
        m = branch_pattern.match(branch)
        if not m:
            new_branches.append(branch)  # 保留无法解析的分支
            continue

        indent = m.group(1)
        mode = m.group(2)
        execution = m.group(3).rstrip()  # 保留执行语句（包括可能的表达式）
        comma = m.group(4)
        comment = m.group(5) or ""

        # 提取原值并替换为成员名
        value_match = re.search(r"\b(\d+|0x[0-9a-fA-F]+)\b", mode)
        if value_match:
            original_value = value_match.group(1)
            member = value_to_member.get(original_value)
            if member:
                # 替换值部分（保留原有格式）
                new_pattern = re.sub(
                    r"\b{}\b".format(re.escape(original_value)),
                    f"{enum_name}::{member}",
                    mode,
                    count=1
                )
                branch = f"{indent}{new_pattern} => {execution}{comma}{comment}"

        new_branches.append(branch)

    # 重建 match 语句块
    new_body = ",\n".join(new_branches)
    match_tail += "// end of enum replacement\n"
    new_content = re.sub(pattern, f"{match_head}{new_body}{match_tail}", content, count=1)
    return new_content

def process_value_definition(content, members, enum_name):
    # 1. 构建匹配 enum 定义的正则，避免修改定义内部
    enum_def_pattern = re.compile(
        r'enum\s+' + re.escape(enum_name) + r'\s*\{[^}]*' + 
        r'(?:' + '|'.join(map(re.escape, members)) + r')[^}]*\}',
        re.DOTALL
    )
    
    # 2. 找到所有 enum 定义的位置
    skip_ranges = []
    for match in enum_def_pattern.finditer(content):
        skip_ranges.append((match.start(), match.end()))
    
    # 3. 为每个成员构建替换正则
    processed_code = content
    for member in members:
        pattern = re.compile(
            r'(?<!::)\b' + re.escape(member) + r'\b' +  # 成员名，前面不能有 ::
            r'(?!\s*=\s*\d)'  # 排除 enum 定义中的 = 0 情况
        )
        
        # 4. 执行替换，跳过 enum 定义部分
        last_pos = 0
        new_code_parts = []
        for start, end in skip_ranges:
            # 添加非 enum 定义部分
            segment = processed_code[last_pos:start]
            new_segment = pattern.sub(f'{enum_name}::{member}', segment)
            new_code_parts.append(new_segment)
            
            # 保留 enum 定义部分不变
            new_code_parts.append(processed_code[start:end])
            last_pos = end
        
        # 添加最后一部分
        segment = processed_code[last_pos:]
        new_segment = pattern.sub(f'{enum_name}::{member}', segment)
        new_code_parts.append(new_segment)
        
        processed_code = ''.join(new_code_parts)
    
    return processed_code

def process_value_cast(content, value_name):
    pattern = r"\{" + re.escape(value_name) + r"\} as libc::c_uint"
    replacement = r"\g<0>.to_libc_c_uint()"
        
    new_content = re.sub(pattern, replacement, content)
    return new_content

def extract_keywords(content, enum_name):
    # 构建正则表达式，匹配 mut {value_name} : {enum_name}，包含变量定义和函数参数
    pattern = r"mut (\w+)\s*:\s*" + re.escape(enum_name)
    
    # 查找所有匹配的变量名 (value_name)
    matches = re.findall(pattern, content)
    
    # 返回匹配的关键词数组
    return matches

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

def transfer_struct_rust(proj_path, path_to_output):
    proj_files = get_rs_files(proj_path)
    output_files = get_rs_files(path_to_output)

    files = {}

    for file in proj_files:
        files[os.path.basename(file)] = file

    for file in output_files:
        files[os.path.basename(file)] = file

    rust_files = list(files.values())

    for file in rust_files:
        with open(file, 'r', encoding='utf-8') as f:
            content = f.read()

        # 处理结构体并替换文本
        modified_content = process_struct_in_file(content)

        # 确保输出目录存在
        os.makedirs(path_to_output, exist_ok=True)
        
        # 生成新的文件路径
        output_file_path = os.path.join(path_to_output, os.path.basename(file))

        # 将修改后的内容写入新文件
        with open(output_file_path, 'w', encoding='utf-8') as f:
            f.write(modified_content)

def process_struct_in_file(content):
    result = subprocess.run(
        ["./rust_struct_processer/target/release/rust_struct_processer"],
        input=content,
        capture_output=True,
        text=True
    )

    if result.returncode == 0:
        return result.stdout.strip()
    else:
        return content

def get_rs_files(proj_path):
    rust_files = []
    for root, dirs, files in os.walk(proj_path):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    return rust_files   

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

    transfer_struct_rust(proj_path, path_to_output)


# make_enum_code_c()
# make_enum_code_rust()
# transfer_enum_rust()
# refactoring()

if __name__ == "__main__":
    if len(sys.argv) > 2:
        print("usage: refactoring program path or default path: c_prog/")
    if len(sys.argv) == 2:
        proj_path = sys.argv[1]
        refactoring(proj_path)

    else:
        for prog in pre_process.prog_list:
            proj_path = pre_process.get_prog_path(prog)
            refactoring(proj_path)