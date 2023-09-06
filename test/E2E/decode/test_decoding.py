import subprocess
import os


def test_torrent_to_json():
    # Execute command
    try:
        # Change the working directory to the parent folder
        os.chdir(os.path.dirname(get_local_path()))

        # Run the command and capture the output
        completed_process = subprocess.run(
            'torrentify test/test_files/inputs/torrent/debian.iso.torrent test/test_files/outputs/json/debian.iso json',
            shell=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE)

        # Check if the command was successful (return code 0)
        if completed_process.returncode == 0:
            print(completed_process.stdout)
        else:
            print("Command failed with the following error message:")
            print(completed_process.stderr)
    except Exception as e:
        print(f"An error occurred: {str(e)}")
    # Compare output
    compare_files_by_bytes("test/test_files/inputs/json/debian.iso.json",
                           "test/test_files/outputs/json/debian.torrent")


def compile():
    try:
        # Change the working directory to the parent folder
        os.chdir(os.path.dirname(get_local_path()))

        # Run the command and capture the output
        completed_process = subprocess.run(
            'cargo build',
            shell=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE)

        # Check if the command was successful (return code 0)
        if completed_process.returncode == 0:
            assert(completed_process.returncode == 0)
            print("Compiled successfully")
            print(completed_process.stdout)
        else:
            print("Command failed with the following error message:")
            print(completed_process.stderr)
    except Exception as e:
        print(f"An error occurred: {str(e)}")


def test_suite_decode():
    compile()
    test_torrent_to_json()

# HELPERS


def get_local_path():
    return os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))


def compare_files_by_bytes(file1_path, file2_path):
    try:
        # Open both files in binary mode
        with open(file1_path, 'rb') as file1, open(file2_path, 'rb') as file2:
            # Read the contents of both files as bytes
            content1 = file1.read()
            content2 = file2.read()

            # Compare the contents byte by byte
            if content1 == content2:
                return True
            else:
                return False

    except FileNotFoundError:
        print("One or both of the files does not exist.")
        return False


# Provide the file paths you want to compare
file1_path = 'file1.bin'
file2_path = 'file2.bin'

if compare_files_by_bytes(file1_path, file2_path):
    print("Both files have the same content.")
else:
    print("Files have different content.")


print(f"cwd: {get_local_path()}")
