import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Chat 1.0

ApplicationWindow {
    id: root
    width: 600
    height: 500
    visible: true
    title: "Markdown Chat"

    // Модель чата
    property var chatModel: ChatModel {}

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        // Список сообщений
        ScrollView {
            id: messageScroll
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ListView {
                id: messageList
                model: chatModel.messages
                spacing: 10
                verticalLayoutDirection: ListView.BottomToTop

                delegate: Rectangle {
                    width: ListView.view.width - 20
                    height: messageColumn.height + 20
                    color: index % 2 === 0 ? "#e3f2fd" : "#f3e5f5"
                    radius: 10
                    anchors.horizontalCenter: parent.horizontalCenter

                    Column {
                        id: messageColumn
                        width: parent.width - 20
                        anchors.centerIn: parent
                        spacing: 5

                        Repeater {
                            model: items

                            delegate: Text {
                                width: parent.width
                                wrapMode: Text.Wrap
                                text: content
                                font: {
                                    switch(type) {
                                    case 1: return Qt.font({family: "Arial", bold: true, pointSize: 16})
                                    case 2: return Qt.font({family: "Arial", bold: true, pointSize: 14})
                                    case 3: return Qt.font({family: "Arial", bold: true, pointSize: 12})
                                    case 4: return Qt.font({family: "Monospace", pointSize: 10})
                                    case 5: return Qt.font({family: "Arial", italic: true, pointSize: 11})
                                    default: return Qt.font({family: "Arial", pointSize: 11})
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Панель ввода сообщения
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Math.min(messageEdit.implicitHeight + 20, 150)
            color: "#f5f5f5"
            border.color: "#cccccc"

            RowLayout {
                anchors.fill: parent
                anchors.margins: 10                

                Button {
                    text: "Edit"
                    onClicked: editDialog.open()
                }

                ScrollView {
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    TextArea {
                        id: messageEdit
                        placeholderText: "Type your message (Markdown supported)..."
                        wrapMode: TextArea.Wrap
                        selectByMouse: true
                    }
                }

                Button {
                    text: "Send"
                    onClicked: {
                        if (messageEdit.text.trim() !== "") {
                            chatModel.send_message(messageEdit.text)
                            messageEdit.clear()
                        }
                    }
                }
            }
        }
    }

    // Диалог редактирования
    Dialog {
        id: editDialog
        modal: true
        title: "Edit Message"
        standardButtons: Dialog.Ok | Dialog.Cancel

        width: root.width * 0.8
        height: root.height * 0.6

        ScrollView {
            anchors.fill: parent
            TextArea {
                id: largeEdit
                text: messageEdit.text
                wrapMode: TextArea.Wrap
                selectByMouse: true
            }
        }

        onAccepted: messageEdit.text = largeEdit.text
    }

    // Обработка сигналов
    Connections {
        target: chatModel
        onSend_message: {
            var parsed = chatModel.parse_markdown(message)
            chatModel.add_message(parsed)
        }
    }
}