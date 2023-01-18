from PyQt6 import QtGui

DATA_FILE = "../data/res.txt"
TASK_LIST = []

with open(DATA_FILE) as f:
    t = f.readline()
    TASK_LIST.append(t)

def create_model() -> QtGui.QStandardItemModel:
    model = QtGui.QStandardItemModel()
    for task in TASK_LIST:
        item = QtGui.QStandardItem(task)
        model.appendRow(item)
    return model
