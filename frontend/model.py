from PyQt6 import QtGui
from typing import List

DATA_FILE = "../data/file.txt"

with open(DATA_FILE, 'r') as f:
    TASK_LIST :List[str] = [str(line).rstrip() for line in f]

def create_model() -> QtGui.QStandardItemModel:
    model = QtGui.QStandardItemModel()
    for task in TASK_LIST:
        item = QtGui.QStandardItem(task)
        model.appendRow(item)
    return model
