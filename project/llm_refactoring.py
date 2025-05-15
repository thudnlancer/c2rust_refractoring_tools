import openai
import httpx
import os
import sys
import re
import time
import pre_process

# 构造C到Rust的翻译提示
def construct_translation_prompt_from_c(c_code):
    prompt = f"""请将以下C代码翻译为等效的安全Rust代码。

C代码:
```c
{c_code}
```

请确保翻译后的Rust代码:
1. 保持与原C代码相同的功能
2. 遵循Rust的安全实践
3. 使用标准Rust库替代C库函数
4. 避免使用unsafe代码块
5. 处理好内存管理，使用Rust的所有权系统
6. 为可能出错的操作提供适当的错误处理
7. 保留原始代码的注释和逻辑结构

请直接返回Rust代码，不要包含任何解释或其他信息。请使用以下格式返回Rust代码:

```rust
// 你的Rust代码
```
"""
    return prompt

# 构造C2Rust到Rust的翻译提示
def construct_translation_prompt_from_rust(rust_code):
    prompt = f"""请将以下由C2Rust翻译的Rust代码重构为等效的安全Rust代码。

Rust代码:
```c2rust
{rust_code}
```

请确保翻译后的Rust代码:
1. 保持与原代码相同的功能
2. 遵循Rust的安全实践
3. 避免使用unsafe代码块
4. 集中处理枚举和结构体的不安全使用

请直接返回Rust代码，不要包含任何解释或其他信息。请使用以下格式返回Rust代码:

```rust
// 你的Rust代码
```
"""
    return prompt

# 初始化OpenAI实例
def init_openai_instance():
    custom_base_url = 'https://api.hunyuan.cloud.tencent.com/v1'
    custom_api_key = 'sk-Z93zhCrsIt5uSmJN9qbpxmWrHJnoW9tBc46azLVaXN64wWCa'
    custom_http_client = httpx.Client(
        base_url=custom_base_url,
        follow_redirects=True,
    )
    
    # 设置OpenAI实例
    openai_instance = openai.OpenAI(
        base_url=custom_base_url,
        api_key=custom_api_key,
        http_client=custom_http_client
    )

    return openai_instance

# 调用LLM API
def call_llm_api(prompt):
    openai_instance = init_openai_instance()
    
    try:
        response = openai_instance.chat.completions.create(
            model="hunyuan-code",
            messages=[
                {"role": "system", "content": "你是一位精通C和Rust编程语言的专家，擅长将C代码安全高效地转换为Rust代码。"},
                {"role": "user", "content": prompt}
            ]
        )
        
        # 提取生成的代码
        generated_code = response.choices[0].message.content
        return generated_code
    except Exception as e:
        print(f"调用LLM API时出现异常: {e}")
        time.sleep(3)  # 出错时等待一段时间
        return ""
    
# 从LLM响应中提取Rust代码
def extract_rust_code(text):
    # 尝试从Markdown代码块中提取
    rust_code_pattern = r'```rust\n(.*?)\n```'
    matches = re.findall(rust_code_pattern, text, re.DOTALL)
    
    if matches:
        return matches[0]
    
    # 如果没有明确的代码块，尝试提取任何代码块
    code_block_pattern = r'```\n(.*?)\n```'
    matches = re.findall(code_block_pattern, text, re.DOTALL)
    if matches:
        return matches[0]
    
    # 最后尝试直接提取所有内容
    lines = text.split('\n')
    code_lines = []
    in_code = False
    
    for line in lines:
        if line.strip().startswith("```") and not in_code:
            in_code = True
            continue
        elif line.strip() == "```" and in_code:
            in_code = False
            continue
        
        if in_code or (not line.startswith("```") and not re.match(r'^[A-Za-z\s]*:', line)):
            code_lines.append(line)
    
    return "\n".join(code_lines)

import os
from concurrent.futures import ThreadPoolExecutor
from functools import partial

def ensure_directory_exists(directory_path):
    """Ensure that a directory exists, creating it if necessary."""
    if not os.path.exists(directory_path):
        os.makedirs(directory_path)

def process_single_c_file(c_file, content, output_dir):
    """Process a single C file and generate corresponding Rust code."""
    print(f"Processing file: {c_file}")
    
    # Generate Rust code from C
    prompt = construct_translation_prompt_from_c(content)
    response = call_llm_api(prompt)
    rust_code = extract_rust_code(response)
    
    if not rust_code:
        print(f"Failed to extract Rust code from response for file: {c_file}")
        return
    
    # Save generated Rust code
    rust_filename = os.path.basename(c_file).replace('.c', '.rs')
    rust_file_path = os.path.join(output_dir, rust_filename)
    
    try:
        with open(rust_file_path, 'w', encoding='utf-8') as rust_file:
            rust_file.write(rust_code)
        print(f"Generated Rust code saved to: {rust_file_path}")
    except IOError as e:
        print(f"Error saving Rust file {rust_file_path}: {str(e)}")

def process_single_rust_file(rust_file, content, output_dir):
    """Process a single Rust file and generate corresponding Rust code."""
    print(f"Processing file: {rust_file}")
    
    # Generate Rust code from C
    prompt = construct_translation_prompt_from_rust(content)
    response = call_llm_api(prompt)
    rust_code = extract_rust_code(response)
    
    if not rust_code:
        print(f"Failed to extract Rust code from response for file: {rust_file}")
        return
    
    # Save generated Rust code
    rust_filename = os.path.basename(rust_file)
    rust_file_path = os.path.join(output_dir, rust_filename)
    
    try:
        with open(rust_file_path, 'w', encoding='utf-8') as rust_file:
            rust_file.write(rust_code)
        print(f"Generated Rust code saved to: {rust_file_path}")
    except IOError as e:
        print(f"Error saving Rust file {rust_file_path}: {str(e)}")

def refactoring(project_path, max_workers=8):
    """
    Refactor C code to Rust by processing C files in a project and generating corresponding Rust files.
    Uses multi-threading to parallelize LLM API calls.
    
    Args:
        project_path (str): Path to the project directory containing C files to be refactored.
        max_workers (int): Maximum number of threads to use for parallel processing.
    """
    # Initialize directories
    project_name = os.path.basename(project_path)
    output_dirs = {
        'c_output': os.path.join('llm_from_c_output_hunyuan', project_name),
        'rust_output': os.path.join('llm_from_rust_output_hunyuan', project_name)
    }
    
    # Create necessary directories
    for dir_path in ['llm_from_c_output_hunyuan', 'llm_from_rust_output_hunyuan'] + list(output_dirs.values()):
        ensure_directory_exists(dir_path)
    
    # Process C files
    c_files, rs_files = find_matching_c_rs_files(project_path)
    combined_contents = read_and_concat_c_h_files(c_files)
    
    # Create a partial function with fixed output_dir parameter
    process_c_func = partial(process_single_c_file, output_dir=output_dirs['c_output'])
    process_rust_func = partial(process_single_rust_file, output_dir=output_dirs['rust_output'])
    
    # Use ThreadPoolExecutor to parallelize the processing
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        # Submit all files for processing
        futures = []
        for c_file, content in combined_contents.items():
            future = executor.submit(process_c_func, c_file, content)
            futures.append(future)
        
        for rs_file in rs_files:
            with open(rs_file, 'r', encoding='utf-8', errors='ignore') as f:
                content_file = f.read()
                future = executor.submit(process_rust_func, rs_file, content_file)
                futures.append(future)
    
        # Wait for all tasks to complete (optional, for error handling)
        for future in futures:
            try:
                future.result()  # This will re-raise any exceptions from the thread
            except Exception as e:
                print(f"Error processing file: {str(e)}")

    



def read_and_concat_c_h_files(c_files):
    combined_contents = {}

    for c_file in c_files:
        base_name = os.path.splitext(os.path.basename(c_file))[0]
        h_file = os.path.join(os.path.dirname(c_file), f"{base_name}.h")

        content = ""
        if os.path.exists(h_file):
            with open(h_file, 'r', encoding='utf-8', errors='ignore') as hf:
                content += hf.read() + "\n"

        with open(c_file, 'r', encoding='utf-8', errors='ignore') as cf:
            content += cf.read()

        combined_contents[c_file] = content

    return combined_contents

def find_matching_c_rs_files(directory):
    c_files = []
    rs_files = []
    prog = os.path.basename(directory)

    # 收集所有 .rs 文件的文件名（不含扩展名），以便快速匹配
    rs_basenames = {}
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                name = os.path.splitext(file)[0]
                rs_basenames[name] = os.path.join(root, file)

    # 查找每一个 .c 文件是否有对应的 .rs 文件
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.c'):
                name = os.path.splitext(file)[0]
                alt_name = name.replace('-', '_')
                if name in rs_basenames or alt_name in rs_basenames:
                    c_output_file = os.path.join('llm_from_c_output_hunyuan', prog, os.path.basename(file).replace('.c', '.rs'))
                    if not os.path.exists(c_output_file):
                        c_files.append(os.path.join(root, file))

                    rs_file_path = rs_basenames.get(name) or rs_basenames.get(alt_name)
                    rust_output_file = os.path.join('llm_from_rust_output_hunyuan', prog, os.path.basename(rs_file_path))
                    if not os.path.exists(rust_output_file):                      
                        rs_files.append(rs_file_path)

    return c_files, rs_files

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