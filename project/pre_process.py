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

def find_file(file_name, directory):
    for root, dirs, files in os.walk(directory):
        if file_name in files:
            return os.path.join(root, file_name)
    return None

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

                if struct_name not in struct_dict[use_file_name]:
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
            
                if enum_name not in enum_dict[use_file_name]:
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

        for filename in structs.keys():
            file_name_without_ext = os.path.splitext(filename)[0]
            rust_file_name = file_name_without_ext + '.rs'
            rust_file = find_file(rust_file_name, path_to_prog)
            if rust_file:
                if rust_file not in struct_list:
                    struct_list.append(rust_file)
            else:
                rust_file_name = file_name_without_ext.replace('-', '_')
                rust_file = find_file(rust_file_name, path_to_prog)
                if rust_file:
                    if rust_file not in struct_list:
                        struct_list.append(rust_file)

        for filename in enums.keys():
            file_name_without_ext = os.path.splitext(filename)[0]
            rust_file_name = file_name_without_ext + '.rs'
            rust_file = find_file(rust_file_name, path_to_prog)
            if rust_file:
                if rust_file not in enum_list:
                    enum_list.append(rust_file)
            else:
                rust_file_name = file_name_without_ext.replace('-', '_') + '.rs'
                rust_file = find_file(rust_file_name, path_to_prog)
                if rust_file:
                    if rust_file not in enum_list:
                        enum_list.append(rust_file)
                    
        if not os.path.exists('candidate_rust/' + prog):
            os.makedirs('candidate_rust/' + prog)
        
        with open('candidate_rust/' + prog + '/struct.json', 'w') as json_file:
            json.dump(struct_list, json_file)
        with open('candidate_rust/' + prog + '/enum.json', 'w') as json_file:
            json.dump(enum_list, json_file)

def ensure_compile_arguments(compile_command):
    """
    检查并补全 compile_commands.json 的 arguments 列表。
    """
    arguments = compile_command['arguments']
    directory = compile_command['directory']

    arguments[0] = 'clang'

    return arguments

def get_included_files(file_path, compile_command):
    """
    提取 C 文件中所有的引用文件（#include 文件），包括条件编译后的结果。
    """

    index = clang.cindex.Index.create()

     # 获取目录和参数
    arguments = ensure_compile_arguments(compile_command)
    
    # 解析文件
    try:
        translation_unit = index.parse(file_path, args=arguments, unsaved_files=None, options=0)
    except Exception as e:
        print(f"Failed to parse {file_path}: {e}")
        return None

    # 收集引用文件
    included_files = []
    for inclusion in translation_unit.get_includes():
        if inclusion.include:
            included_files.append(str(inclusion.include))
    
    return included_files

def analyze_project_with_compile_commands(project_path, compile_commands_path):
    """
    利用 compile_commands.json 分析 C 项目中每个文件的 #include 文件。
    """
    # 加载 compile_commands.json
    compile_commands = load_compile_commands(compile_commands_path)
    includes_map = {}

    # 遍历项目目录
    for root, _, files in os.walk(project_path):
        for file in files:
            if file.endswith('.c'):
                file_path = os.path.abspath(os.path.join(root, file))
                
                # 获取对应的编译命令
                compile_command = compile_commands.get(file_path)
                if not compile_command:
                    print(f"No compile command found for {file_path}")
                    continue

                print(f"Analyzing {file_path}...")
                included_files = get_included_files(file_path, compile_command)
                if included_files is not None:
                    includes_map[file_path] = included_files

    return includes_map

def load_compile_commands(compile_commands_path):
    """
    加载 compile_commands.json 文件，返回文件路径到编译参数的映射。
    """
    with open(compile_commands_path, 'r') as f:
        compile_commands = json.load(f)
    
    commands_map = {}
    base_dir = os.path.dirname(compile_commands_path)

    for entry in compile_commands:
        file_path = os.path.abspath(os.path.join(base_dir, entry['file']))
        directory = entry['directory']
        command = entry['command'] if 'command' in entry else entry['arguments']
        
        # 将命令拆分为列表（如果是单个字符串）
        if isinstance(command, str):
            command = command.split()
        
        # 组合为绝对路径和完整参数
        commands_map[file_path] = {
            'directory': directory,
            'arguments': command
        }
    return commands_map

def make_c_include_list():
    if not os.path.exists('c_include_list'):
        os.makedirs('c_include_list')
    
    for prog in prog_list:
        # 设置项目路径和 compile_commands.json 路径
        project_path = get_prog_path(prog)
        compile_commands_path = os.path.join(project_path, "compile_commands.json")

        # 分析项目中的 #include 文件
        project_includes = analyze_project_with_compile_commands(project_path, compile_commands_path)

        if not os.path.exists('c_include_list/' + prog):
            os.makedirs('c_include_list/' + prog)
        
        # 保存结果到指定路径
        with open('c_include_list/' + prog + '/c_include_list.json', 'w') as f:
            json.dump(project_includes, f, indent=4)



# make_data_csv()
# make_location_c_json()
# make_candidate_c_list()
# make_candidate_rust_list()
make_c_include_list()