UI_FILE = "view.ui"
NEW_FILE = "../data/new_file.txt"
OLD_FILE = "../data/file.txt"
FLAG_FILE = "../data/flag.txt"

import os 
from PyQt6 import QtGui, QtWidgets, uic
from PyQt6.QtCore import QModelIndex

from model import create_model


class TodoList(QtWidgets.QMainWindow):
    def __init__(self) -> None:
        QtWidgets.QMainWindow.__init__(self)
        uic.loadUi(UI_FILE, self)
        self.task_list_model = create_model()
        self.task_list.setModel(self.task_list_model)
        self.task_list.selectionModel().selectionChanged.connect(
            self.on_change_selection
        )
        self.todo_edit.returnPressed.connect(self.on_add_task)
        self.delete_button.pressed.connect(self.on_delete_task)
        self.refresh_button.pressed.connect(self.on_refresh)
        self.store_button.pressed.connect(self.on_store)

    def on_add_task(self) -> None:
        text: str = self.todo_edit.text()
        if not text:
            return
        self.task_list_model.appendRow(QtGui.QStandardItem(text))
        self.todo_edit.clear()

    def on_delete_task(self) -> None:
        indexes: list[QModelIndex] = self.task_list.selectedIndexes()
        if not indexes:
            return
        self.task_list_model.removeRow(indexes[0].row())
        self.task_list.clearSelection()

    def on_change_selection(self) -> None:
        indexes: list[QModelIndex] = self.task_list.selectedIndexes()
        self.delete_button.setEnabled(bool(indexes))

    def on_refresh(self) -> None:
        self.task_list_model = create_model()
        self.task_list.setModel(self.task_list_model)

    def on_store(self) -> None:
        with open(NEW_FILE, 'w') as f:
            for index in range(self.task_list_model.rowCount()):
                task :str = str(self.task_list_model.item(index))
                f.write(task)
                f.write('\n')
        with open(FLAG_FILE, '+') as f:
            if f.readlines()[0].rstrip() == "BACKEND STOP":
                f.truncate(0)
                f.write("BACKEND START")
        if os.path.exists(OLD_FILE):
            os.remove(OLD_FILE)
        os.rename(NEW_FILE, OLD_FILE)

