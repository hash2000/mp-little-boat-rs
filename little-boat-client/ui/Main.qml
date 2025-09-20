import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts
import Chat

ApplicationWindow {
    id: root
    width: 600
    height: 500
    visible: true
    title: "Markdown Chat"
    color: Style.backgroundColor

    ChatMessagesListModel {
        id: messagesListModel
    }

    // Панель переключения темы
    Rectangle {
        id: themeSwitcher
        width: 40
        height: 40
        radius: 20
        color: Style.darkTheme ? Style.darkMessageEven : Style.lightMessageEven
        anchors {
            top: parent.top
            right: parent.right
            margins: 10
        }

        Text {
            text: Style.darkTheme ? "☀️" : "🌙"
            font.pointSize: 16
            anchors.centerIn: parent
        }

        MouseArea {
            anchors.fill: parent
            onClicked: Style.darkTheme = !Style.darkTheme
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        // Список сообщений
        ScrollView {
            id: messageScroll
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ScrollBar.vertical: ScrollBar {
                background: Rectangle { color: Style.scrollbarBackgroundColor }
                contentItem: Rectangle { color: Style.scrollbarHandleColor; radius: 3 }
            }

            ListView {
                id: messageList
                model: messagesListModel.messages
                spacing: 10
                verticalLayoutDirection: ListView.BottomToTop

                delegate: Rectangle {
                    width: ListView.view.width - 20
                    height: messageColumn.height + 20
                    color: index % 2 === 0 ? Style.messageEvenColor : Style.messageOddColor
                    radius: 10
                    anchors.horizontalCenter: parent.horizontalCenter

                    Column {
                        id: messageColumn
                        width: parent.width - 20
                        anchors.centerIn: parent
                        spacing: 5

                        Repeater {
                            model: itemType && content ? itemType.length : 0

                            delegate: Text {
                                width: parent.width
                                wrapMode: Text.Wrap
                                text: content[index] || ""
                                color: Style.textColor
                                font: {
                                    switch(itemType[index]) {
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
            color: "transparent"

            RowLayout {
                anchors.fill: parent
                anchors.margins: 10
                spacing: 10

                ScrollView {
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    TextArea {
                        id: messageEdit
                        placeholderText: "Type your message ..."
                        wrapMode: TextArea.Wrap
                        selectByMouse: true
                        color: Style.textColor
                        placeholderTextColor: Style.placeholderColor

                        background: Rectangle {
                            color: Style.inputBackgroundColor
                            radius: 8
                            border.width: 0
                        }
                    }
                }

                Button {
                    text: "Send"
                    background: Rectangle {
                        color: Style.buttonBackgroundColor
                        radius: 8
                    }
                    contentItem: Text {
                        text: parent.text
                        color: Style.buttonTextColor
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    onClicked: {
                        if (messageEdit.text.trim() !== "") {
                            messagesListModel.send_message(messageEdit.text)
                            messageEdit.clear()
                        }
                    }
                }
            }
        }
    }

    // Обработка сигналов
    Connections {
        target: messagesListModel
        function onSend_message(message) {
            console.log("Message sent to backend:", message)
        }
    }
}