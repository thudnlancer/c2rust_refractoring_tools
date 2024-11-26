import csv
import json
import os
import clang.cindex
from collections import defaultdict

clang.cindex.Config.set_library_file('/usr/lib/llvm-6.0/lib/libclang.so')
index = clang.cindex.Index.create()

prog_list = ['binn', 'cflow', 'compton', 'cpio-2.14', 'enscript-1.6.1', 'findutils-4.9.0', 'gawk-5.2.2', 'glpk-5.0', 'grep-3.11', 'gsl-2.7.1', 'gzip-1.12', 'hiredis', 
             'libtree', 'libzahl', 'make-4.4.1', 'minilisp', 'mtools-4.0.45', 'nettle-3.10', 'pth-2.0.7', 'rcs-5.10.0', 'sed-4.9', 'tar-1.34', 'twemproxy', 'webdis', 'wget-1.21.4']

def get_prog_path(prog_name):
    return '../c_prog/' + prog_name

def get_location_path(prog_name):
    return 'location_c/' + prog_name

def get_candidate_path(prog_name):
    return 'candidate_c/' + prog_name

def get_data_list(prog, stats):
    return [prog, stats['files'], stats['LOC'], stats['structs'], stats['unions'], stats['enums'], stats['functions']]

def walk_through_directory(directory):
    if not os.path.exists(directory):
        print(f"错误：路径不存在 - {directory}")
        return
    if not os.path.isdir(directory):
        print(f"错误：路径不是一个目录 - {directory}")
        return

    for dirpath, dirnames, filenames in os.walk(directory):
        if not dirnames and not filenames:
            print(f"目录是空的或没有权限访问：{dirpath}")
        else:
            print(f"当前目录: {dirpath}")
            for dirname in dirnames:
                print(f"子目录: {os.path.join(dirpath, dirname)}")
            for filename in filenames:
                print(f"文件: {os.path.join(dirpath, filename)}")

def count_c_prog(path_to_project):
    stats = {
    'structs': 0,
    'unions': 0,
    'enums': 0,
    'functions': 0,
    'files': 0,
    'LOC': 0
    }

    for root, dirs, files in os.walk(path_to_project):
        for file in files:
            if file.endswith(('.c', '.h')):
                file_path = os.path.join(root, file)
                with open(file_path, 'r', errors='ignore') as f:
                    # 计算每个文件的行数，并累加到总行数中
                    stats['LOC'] += sum(1 for line in f if line.strip())
                    
                stats['files'] += 1
                # 解析文件
                translation_unit = index.parse(os.path.join(root, file))
                # 遍历AST
                for cursor in translation_unit.cursor.get_children():
                    if cursor.kind == clang.cindex.CursorKind.STRUCT_DECL:
                        stats['structs'] += 1
                    elif cursor.kind == clang.cindex.CursorKind.UNION_DECL:
                        stats['unions'] += 1
                    elif cursor.kind == clang.cindex.CursorKind.ENUM_DECL:
                        stats['enums'] += 1
                    elif cursor.kind == clang.cindex.CursorKind.FUNCTION_DECL:
                        stats['functions'] += 1

    return stats

def make_data_csv():
    with open('data.csv', 'w', newline='') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(['Name', 'File_Num', 'LOC', 'Struct_Num', 'Union_Num', 'Enum_Num', 'Func_Num'])  # 写入表头
        for prog in prog_list:
            path_to_project = get_path_name(prog)
            stats = count_c_prog(path_to_project)
            writer.writerow(get_data_list(prog, stats))

def location_to_dict(location, file):
    # 获取文件、行、列等信息
    def_file, line, column, offset = location._get_instantiation()
    use_file = file
    return {
        'def_file': def_file.name,
        'use_file': use_file,
        'line': line,
        'column': column,
        'offset': offset
    }

def make_json_helper(dict, name, location, file):
    if name in dict:
        dict[name].append(location_to_dict(location,file))
    else:
        dict[name] = [location_to_dict(location, file)]

def make_location_c_json():
    if not os.path.exists('location_c'):
        os.makedirs('location_c')

    for prog in prog_list:
        struct_dict = {}
        union_dict = {}
        enum_dict = {}
        func_dict = {}
        path_to_project = get_prog_path(prog)

        for root, dirs, files in os.walk(path_to_project):
            for file in files:
                if file.endswith(('.c', '.h')):                   
                    # 解析文件
                    translation_unit = index.parse(os.path.join(root, file))
                    # 遍历AST
                    for cursor in translation_unit.cursor.get_children():
                        if cursor.kind == clang.cindex.CursorKind.STRUCT_DECL:
                            make_json_helper(struct_dict, cursor.spelling, cursor.location, file)
                        elif cursor.kind == clang.cindex.CursorKind.UNION_DECL:
                            make_json_helper(union_dict, cursor.spelling, cursor.location, file)
                        elif cursor.kind == clang.cindex.CursorKind.ENUM_DECL:
                            make_json_helper(enum_dict, cursor.spelling, cursor.location, file)
                        elif cursor.kind == clang.cindex.CursorKind.FUNCTION_DECL:
                            make_json_helper(func_dict, cursor.spelling, cursor.location, file)

        if not os.path.exists('location_c/' + prog):
            os.makedirs('location_c/' + prog)
        
        with open('location_c/' + prog + '/struct.json', 'w') as json_file:
            json.dump(struct_dict, json_file)
        with open('location_c/' + prog + '/union.json', 'w') as json_file:
            json.dump(union_dict, json_file)
        with open('location_c/' + prog + '/enum.json', 'w') as json_file:
            json.dump(enum_dict, json_file)
        with open('location_c/' + prog + '/func.json', 'w') as json_file:
            json.dump(func_dict, json_file)

def make_candidate_c_list():
    if not os.path.exists('candidate_c'):
        os.makedirs('candidate_c')
    
    for prog in prog_list:
        path_to_location = get_location_path(prog)
        # 初始化结果字典
        struct_dict = defaultdict(list)
        enum_dict = defaultdict(list)

        struct_json_file = path_to_location + '/struct.json'
        enum_json_file = path_to_location + '/enum.json'
        prefix_filter = '../c_prog/' + prog + '/'
        with open(struct_json_file, 'r', encoding='utf-8') as f:
            data = json.load(f)

        for struct_name, paths in data.items():
            for path in paths:
                def_file_path = path.get("def_file", "")
                use_file_name = path.get("use_file", "")

                if not def_file_path.startswith(prefix_filter):
                    continue
            
                # 提取 def_file 的文件名
                def_file_name = os.path.basename(def_file_path)
            
                # 如果 def_file 文件名与 use_file 文件名不同，则记录为使用
                if def_file_name != use_file_name:
                    struct_dict[use_file_name].append(struct_name)
        
        # 转换 defaultdict 为普通字典
        struct_dict = dict(struct_dict)

        with open(enum_json_file, 'r', encoding='utf-8') as f:
            data = json.load(f)

        for enum_name, paths in data.items():
            for path in paths:
                def_file_path = path.get("def_file", "")
                use_file_name = path.get("use_file", "")

                if not def_file_path.startswith(prefix_filter):
                    continue
        
                # 提取 def_file 的文件名
                def_file_name = os.path.basename(def_file_path)
        
                # 如果 def_file 文件名与 use_file 文件名不同，则记录为使用
                if def_file_name != use_file_name:
                    enum_dict[use_file_name].append(enum_name)
        
        # 转换 defaultdict 为普通字典
        enum_dict = dict(enum_dict)
        
        if not os.path.exists('candidate_c/' + prog):
            os.makedirs('candidate_c/' + prog)
        
        with open('candidate_c/' + prog + '/struct.json', 'w') as json_file:
            json.dump(struct_dict, json_file)
        with open('candidate_c/' + prog + '/enum.json', 'w') as json_file:
            json.dump(enum_dict, json_file)

def make_candidate_rust_list():
    if not os.path.exists('candidate_rust'):
        os.makedirs('candidate_rust')
    
    for prog in prog_list:
        path_to_candidate = get_candidate_path(prog)
        path_to_prog = get_prog_path(prog)
        struct_json_file = path_to_candidate + '/struct.json'
        enum_json_file = path_to_candidate + '/enum.json'
        with open(struct_json_file, 'r', encoding='utf-8') as f:
            structs = json.load(f)
        with open(enum_json_file, 'r', encoding='utf-8') as f:
            enums = json.load(f)
        
        # 初始化结果字典
        struct_list = []
        enum_list = []

        for root, _, files in os.walk(path_to_prog):
            for file in files:
                # 检查文件后缀名是否为 `.rs`
                if file.endswith(".rs"):
                    # 提取不包含后缀名的文件名
                    file_name_without_ext = os.path.splitext(file)[0]
                    file_name_with_c = file_name_without_ext + '.c'

                    # 如果文件名与 keys 中的某个值匹配，则记录
                    if file_name_with_c in list(structs.keys()):
                        struct_list.append(os.path.join(root, file))
                    if file_name_with_c in list(enums.keys()):
                        enum_list.append(os.path.join(root, file))
                    
        if not os.path.exists('candidate_rust/' + prog):
            os.makedirs('candidate_rust/' + prog)
        
        with open('candidate_rust/' + prog + '/struct.json', 'w') as json_file:
            json.dump(struct_list, json_file)
        with open('candidate_rust/' + prog + '/enum.json', 'w') as json_file:
            json.dump(enum_list, json_file)

# make_data_csv()
# make_location_c_json()
# make_candidate_c_list()
# make_candidate_rust_list()