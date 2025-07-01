import sys
from PyQt5.QtWidgets import( 
    QApplication, QMainWindow, QWidget, QPushButton, 
    QLineEdit, QLabel, QVBoxLayout, QMessageBox
)

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Port Scanner")
        self.setGeometry(0, 0, 1366, 768)

        self.label = QLabel("Enter IP Address")
        self.input = QLineEdit(self)
        self.input.setPlaceholderText("e.g 192.168.1.0")

        self.sub_button = QPushButton("Scan", self)
        self.sub_button.clicked.connect()


def main():
    app = QApplication([])
    window = MainWindow()
    window.show()
    sys.exit(app.exec_())

if __name__ == "__main__":
    main()