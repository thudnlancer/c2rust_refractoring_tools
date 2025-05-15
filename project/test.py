import os
import shutil
import subprocess
import json
import pre_process
import sys
from pathlib import Path
from contextlib import contextmanager

def delete_rs_files(directory):
    """递归删除目录中的所有 .rs 文件"""
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs") or file.endswith(".toml"):
                os.remove(os.path.join(root, file))
        for dir in dirs:
            delete_rs_files(os.path.join(root, dir))

def run_c2rust_transpile():
    """Run c2rust transpile command with logging."""
    logger = setup_logging("c2rust_transpile.log")
    command = ["c2rust", "transpile", "-e", "compile_commands.json"]
    
    logger.info(f"Running: {' '.join(command)}")
    try:
        result = subprocess.run(
            command,
            capture_output=True,
            text=True,
            check=True
        )
        logger.info("c2rust transpile succeeded")
        logger.debug(f"Output:\n{result.stdout}")
    except subprocess.CalledProcessError as e:
        logger.error(f"c2rust failed with code {e.returncode}")
        logger.error(f"Error output:\n{e.stderr}")
        return False 

def replace_rs_files(file_list, output_dir):
    """根据文件清单替换生成的 .rs 文件"""
    for file_path in file_list:
        file_name = os.path.basename(file_path)
        output_file_path = os.path.join(output_dir, file_name)
        if os.path.exists(output_file_path):
            shutil.copy(output_file_path, file_path)

def run_cargo_build(log_file: str):
    """Run cargo build with output logging."""
    logger = setup_logging(log_file)
    command = ["cargo", "build"]

    # 清空日志文件
    with open(log_file, 'w') as f:
        pass  # 只是打开并立即关闭以清空文件
    
    logger.info(f"Running: {' '.join(command)}")
    try:
        with open(log_file, 'a') as f:
            process = subprocess.Popen(
                command,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1
            )
            
            for line in process.stdout:
                f.write(line)
                logger.info(line.strip())
                
            if process.wait() != 0:
                raise subprocess.CalledProcessError(process.returncode, command)
                
    except subprocess.CalledProcessError as e:
        logger.error(f"cargo build failed with code {e.returncode}")
        return False


def get_file_list(directory):
    rs_files = []
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs"):
                rs_files.append(os.path.join(root, file))
    return rs_files

def setup_logging(log_file: str):
    """Configure logging to both file and console."""
    import logging

    logger = logging.getLogger()
    if log_file:
            try:
                log_path = Path(log_file)
                
                # Create parent directories if they don't exist
                log_path.parent.mkdir(parents=True, exist_ok=True)
                
                # Create empty file if it doesn't exist
                if not log_path.exists():
                    log_path.touch(exist_ok=True)
                
                # Verify file is writable
                if not os.access(log_file, os.W_OK):
                    raise PermissionError(f"No write permissions for log file: {log_file}")
                
            except Exception as e:
                logger.error(f"Failed to setup file logging: {str(e)}")
    return logger



@contextmanager
def change_directory(destination):
    """上下文管理器：临时切换目录"""
    original_dir = os.getcwd()
    try:
        os.chdir(destination)
        yield
    finally:
        os.chdir(original_dir)

def main(directory, output_dir):
    log_origin = "../../" + output_dir + "/test_origin.log"
    log_replace = "../../" + output_dir + "/test_replace.log"
    # 1. 递归删除所有的 .rs 文件
    delete_rs_files(directory)
    
    with change_directory(directory):
        # 2. 运行 c2rust transpile -e
        run_c2rust_transpile()
    
    file_list = get_file_list(directory)
    
    # with change_directory(directory):
        # 4. 运行 cargo build
        # run_cargo_build(log_origin)
    
    # 3. 替换生成的 .rs 文件
    replace_rs_files(file_list, output_dir)
    
    with change_directory(directory):
        # 4. 运行 cargo build
        run_cargo_build(log_replace)
    
    # 5. 删除 .rs 文件
    delete_rs_files(directory)

if __name__ == "__main__":
    # 示例路径和文件清单
    args = sys.argv
    if len(args) > 2:
        print("Usage: python test.py <directory> or default path: c_prog/")
        sys.exit(1)
    if len(sys.argv) == 2:
        directory = args[1]
        file_list = get_file_list(directory)
        prog = os.path.basename(directory)
        output_dir = os.path.join('llm_from_rust_output', prog)
        
        main(directory, output_dir)
    else:
        for prog in pre_process.prog_list:
            proj_path = pre_process.get_prog_path(prog)
            output_dir = os.path.join('llm_from_rust_output', prog)
            main(proj_path, output_dir)