import os
import clang.cindex

# 设置libclang库的位置
clang.cindex.Config.set_library_file('/usr/lib/llvm-6.0/lib/libclang.so')

# 创建索引
index = clang.cindex.Index.create()

# 定义统计结果的字典
stats = {
    'structs': 0,
    'unions': 0,
    'enums': 0,
    'functions': 0,
    'files': 0,
    'LOC': 0
}

path_to_project = '~/Desktop/c_prog/binn'

# 遍历项目目录
for root, dirs, files in os.walk(path_to_project):
    for file in files:
        if file.endswith(('.c', '.h')):
            file_path = os.path.join(root, file)
            with open(file_path, 'r') as f:
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

# 打印统计结果
print(f"Files: {stats['files']}")
print(f"Structs: {stats['structs']}")
print(f"Unions: {stats['unions']}")
print(f"Enums: {stats['enums']}")
print(f"Functions: {stats['functions']}")