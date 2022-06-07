import sys
import socket
import re


def backslash_str_to_bytes(text):
    """
    https://stackoverflow.com/questions/38763771/how-do-i-remove-double-back-slash-from-a-bytes-object
    """
    return text.encode().decode('unicode_escape').encode("raw_unicode_escape")


def get_testcases(filename):
    testcases = []

    file = open('key.txt', 'r')
    for line in file.readlines():
        case = line
        line = line.replace('\n', '')
        line = backslash_str_to_bytes(line)
        testcases.append(line)

    return testcases


if __name__ == '__main__':
    filename = 'key.txt'
    testcases = get_testcases(filename)
    print(testcases)

    client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    client.connect("/tmp/RustDesk/pynput_service")

    for i in testcases:
        client.send(i)
