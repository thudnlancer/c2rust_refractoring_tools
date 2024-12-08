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

def extract_enum_info(filename, enum_name=None):
    with open(filename, 'rb') as file:
        raw_data = file.read()
        result = chardet.detect(raw_data)
        encoding = result['encoding']

    # 使用检测到的编码打开文件
    with open(filename, 'r', encoding=encoding) as file:
        code = file.read()
    
    # 匹配枚举的定义（包括匿名枚举）
    enum_pattern = re.compile(r'\b(enum)\s*(\w*)\s*{(.*?)}', re.DOTALL)
    enums = enum_pattern.findall(code)
    
    # 如果指定了枚举名，只查找该枚举，否则提取所有枚举
    enums = [enum for enum in enums if enum[1] == enum_name]
    
    enum_info = []
    
    for enum in enums:
        enum_type, enum_name, enum_body = enum
        enum_members = []
        
        # 提取枚举成员和它们的值
        member_pattern = re.compile(r'(\w+)\s*(?:=\s*(\d+))?')
        members = member_pattern.findall(enum_body)
        
        for member, value in members:
            value = int(value) if value else None  # 默认值为None
            enum_members.append((member, value))
        
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
                'enum_members': enum_members,
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
                enum_info = extract_enum_info(filepath, enum_name)
                if enum_info:
                    file_enums.append(enum_info)
            result[filename] = file_enums

        if not os.path.exists('enum_code_c/' + prog):
            os.makedirs('enum_code_c/' + prog)

        with open('enum_code_c/' + prog + '/enum.json', 'w') as json_file:
            json.dump(result, json_file)

make_enum_code_c()
