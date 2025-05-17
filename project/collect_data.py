import pandas as pd
import subprocess

def extract_values(output):
    lines = output.splitlines()
    uloc = loc = r = None
    for line in lines:
        if "Unsafe lines:" in line:
            uloc = line.split(":")[1].strip()
        elif "Total lines:" in line:
            loc = line.split(":")[1].strip()
        elif "Unsafe percentage:" in line:
            r = line.split(":")[1].strip()
    return uloc, loc, r

def fill_csv_data(input_file, output_file):
    # 读取 CSV 文件
    df = pd.read_csv(input_file)
    
    # 获取第一列的值
    first_column = df.iloc[:, 0]
    
    # 填充每一行的数据
    for index, value in first_column.items():
        # 假设我们需要根据第一列的值来填充其他列
        # 这里只是一个示例，你可以根据实际需求修改逻辑
        if value == "Name" or value == "":
            continue
        else:
            prog = value
            c_prog_path = f"./c_prog/{prog}"
            output_path = f"./output/{prog}"
            llm_from_c_path = f"./llm_from_c_output/{prog}"
            llm_from_rust_path = f"./llm_from_rust_output/{prog}"
            columns = df.columns

            result = subprocess.run(
                ["./unsafe-count/target/release/unsafe-count", c_prog_path],
                capture_output=True,
                text=True
            )
            ULOC, LOC, R = extract_values(result.stdout)
            uloc_col = columns[1]
            loc_col = columns[2]
            r_col = columns[3]

            df.at[index, uloc_col] = ULOC
            df.at[index, loc_col] = LOC
            df.at[index, r_col] = R

            result = subprocess.run(
                ["./unsafe-count/target/release/unsafe-count", output_path],
                capture_output=True,
                text=True
            )
            ULOC, LOC, R = extract_values(result.stdout)
            uloc_col = columns[4]
            loc_col = columns[5]
            r_col = columns[6]

            df.at[index, uloc_col] = ULOC
            df.at[index, loc_col] = LOC
            df.at[index, r_col] = R

            result = subprocess.run(
                ["./unsafe-count/target/release/unsafe-count", llm_from_c_path],
                capture_output=True,
                text=True
            )
            ULOC, LOC, R = extract_values(result.stdout)
            uloc_col = columns[7]
            loc_col = columns[8]
            r_col = columns[9]

            df.at[index, uloc_col] = ULOC
            df.at[index, loc_col] = LOC
            df.at[index, r_col] = R

            result = subprocess.run(
                ["./unsafe-count/target/release/unsafe-count", llm_from_rust_path],
                capture_output=True,
                text=True
            )
            ULOC, LOC, R = extract_values(result.stdout)
            uloc_col = columns[10]
            loc_col = columns[11]
            r_col = columns[12]

            df.at[index, uloc_col] = ULOC
            df.at[index, loc_col] = LOC
            df.at[index, r_col] = R
    
    # 保存填充后的数据到新的 CSV 文件
    df.to_csv(output_file, index=False)
    print(f"填充后的数据已保存到 {output_file}")

# 使用示例
input_csv = "Unsafe_Rate.csv"  # 输入文件路径
output_csv = "result.csv"  # 输出文件路径
fill_csv_data(input_csv, output_csv)