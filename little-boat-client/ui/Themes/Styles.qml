pragma Singleton
import QtQuick

QtObject {
    // Текущая тема
    property bool darkTheme: true

    // Цвета для тёмной темы
    readonly property color darkBackground: "#1e1e2e"
    readonly property color darkMessageEven: "#44475a"
    readonly property color darkMessageOdd: "#6272a4"
    readonly property color darkText: "#f8f8f2"
    readonly property color darkPlaceholder: "#6272a4"
    readonly property color darkInputBackground: "#44475a"
    readonly property color darkButtonBackground: "#50fa7b"
    readonly property color darkButtonText: "#282a36"
    readonly property color darkScrollbarBackground: "#44475a"
    readonly property color darkScrollbarHandle: "#6272a4"

    // Цвета для светлой темы
    readonly property color lightBackground: "#f0f0f0"
    readonly property color lightMessageEven: "#e3f2fd"
    readonly property color lightMessageOdd: "#f3e5f5"
    readonly property color lightText: "#000000"
    readonly property color lightPlaceholder: "#888888"
    readonly property color lightInputBackground: "#ffffff"
    readonly property color lightButtonBackground: "#6200ea"
    readonly property color lightButtonText: "#ffffff"
    readonly property color lightScrollbarBackground: "#e0e0e0"
    readonly property color lightScrollbarHandle: "#bdbdbd"

    // Вычисляемые свойства для текущей темы
    property color backgroundColor: darkTheme ? darkBackground : lightBackground
    property color messageEvenColor: darkTheme ? darkMessageEven : lightMessageEven
    property color messageOddColor: darkTheme ? darkMessageOdd : lightMessageOdd
    property color textColor: darkTheme ? darkText : lightText
    property color placeholderColor: darkTheme ? darkPlaceholder : lightPlaceholder
    property color inputBackgroundColor: darkTheme ? darkInputBackground : lightInputBackground
    property color buttonBackgroundColor: darkTheme ? darkButtonBackground : lightButtonBackground
    property color buttonTextColor: darkTheme ? darkButtonText : lightButtonText
    property color scrollbarBackgroundColor: darkTheme ? darkScrollbarBackground : lightScrollbarBackground
    property color scrollbarHandleColor: darkTheme ? darkScrollbarHandle : lightScrollbarHandle
}