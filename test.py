import os
import glob
import subprocess

# Root directory of the project
project_root = os.getcwd()

# Get all python test files in the project directory
python_files = glob.glob(os.path.join(
    project_root, '**', '*.py'), recursive=True)

# Get the name of the script itsel
script_name = os.path.basename(__file__)
print(script_name)

# Filter the Python files that contain "test" in their names
test_files = [file for file in python_files if 'test' in os.path.basename(
    file).lower() and os.path.basename(file) != script_name]

# Run each test file
for test_file in test_files:
    print(f"Running test file: {test_file}")
    try:
        subprocess.run(['python3', test_file], check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running {test_file}: {e}")
