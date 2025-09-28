import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts
import Chat
import "./Themes"
import "./Chat"

ApplicationWindow {
    id: root
    width: 600
    height: 500
    visible: true
    title: "Chat"
    color: Style.backgroundColor

    RowLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            id: leftPanel
            Layout.preferredWidth: 32
            Layout.fillHeight: true
            color: Style.darkTheme ? Style.darkMessageEven : Style.lightMessageEven

            Button {
                id: settingsButton
                width: parent.width
                height: 32
                anchors.top: parent.top
                anchors.horizontalCenter: parent.horizontalCenter

                background: Rectangle {
                    color: "transparent"
                }

                contentItem: Column {
                    spacing: 2
                    Repeater {
                        model: 3
                        Rectangle {
                            width: 12
                            height: 2
                            color: Style.textColor
                            radius: 1
                        }
                    }
                }

                onClicked: settingsDrawer.open()
            }
        }

        // Основная область с чатом
        ChatView {
            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }

    // Выдвижная панель параметров
    Drawer {
        id: settingsDrawer
        width: 220
        height: parent.height
        edge: Qt.LeftEdge
        dragMargin: 16 // Чтобы можно было открыть от левого края

        ColumnLayout {
            anchors.fill: parent
            spacing: 10

            // Заголовок
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 32
                color: Style.darkTheme ? Style.darkMessageOdd : Style.lightMessageOdd

                Text {
                    text: "Settings"
                    color: Style.textColor
                    font.bold: true
                    font.pointSize: 14
                    anchors.centerIn: parent
                }
            }

            // Переключатель темы
            Button {
                Layout.fillWidth: true
                Layout.preferredHeight: 40
                text: Style.darkTheme ? "☀️ Light Theme" : "🌙 Dark Theme"
                onClicked: Style.darkTheme = !Style.darkTheme
            }

            // Другие настройки можно добавить здесь
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
            }
        }
    }    
}