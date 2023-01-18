from PyQt6 import QtGui

DATA_FILE = "../data/res.txt"

with open(DATA_FILE) as f:
    TASK_LIST = [str(line).rstrip() for line in f]

def create_model() -> QtGui.QStandardItemModel:
    model = QtGui.QStandardItemModel()
    for task in TASK_LIST:
        item = QtGui.QStandardItem(task)
        model.appendRow(item)
    return model
