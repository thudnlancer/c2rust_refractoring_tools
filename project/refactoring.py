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
    
    return code_no_comments

def extract_enum_info_c(filename, enum_name=None):
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
    enums = enum_pattern.findall(code)
    
    # 如果指定了枚举名，只查找该枚举，否则提取所有枚举
    enums = [enum for enum in enums if enum[1] == enum_name]
    
    enum_info = []
    
    for enum in enums:
        enum_type, enum_name, enum_body = enum
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
        
        if enum_name:
            # 获取枚举的定义块的位置
            start_idx = code.find(f"enum {enum_name}")
            end_idx = code.find("}", start_idx)
        else:
            start_idx = code.find(f"enum")
            end_idx = code.find("}", start_idx)
        
        if start_idx >= 0:
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
                filtered_members[member_name] = int(member_value)
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
            file_enums = []
            for enum_name in enum_names:
                enum_info = extract_enum_info_c(filepath, enum_name)
                if enum_info:
                    file_enums.append(enum_info)
            result[filename] = file_enums

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
            filename = os.path.basename(candidate_path)
            file_enums = extract_enum_info_rust(candidate_path)
            result[filename] = file_enums

        if not os.path.exists('enum_code_rust/' + prog):
            os.makedirs('enum_code_rust/' + prog)

        with open('enum_code_rust/' + prog + '/enum.json', 'w') as json_file:
            json.dump(result, json_file)

make_enum_code_c()
make_enum_code_rust()
