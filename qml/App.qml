import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.3
import Rustcode 1.0

ApplicationWindow {
    visible: true
    width: 680
    height: 480
    title: "2048 Game"
    color: Qt.rgba(0.7, 0.7, 0.7, 1)

    App {
        id: app

        Component.onCompleted: {
            init_game();
        }
    }

    GridView {
        id: gridView

        x: 60
        y: 60
        width: 380
        height: 380
        model: 16
        cellWidth: 90
        cellHeight: 90

        delegate: Rectangle {
            property int number: 114514

            function update() {
                number = app.get_board(index);
                if (number == 0) {
                    color = Qt.rgba(0.8, 0.8, 0.8, 1);
                    return ;
                }
                color = Qt.hsva(Math.min(1, Math.log(number, 2) / Math.log(2048, 2) / 3 + 0.66), 0.7, 1, 0.8);
            }

            width: 60
            height: 60
            color: "#11222233"
            radius: 20
            Component.onCompleted: {
                app.game_changed.connect(update);
            }

            Text {
                font.pixelSize: 20
                color:"#bbbbbb"
                font.bold: true
                text: number == 0 ? "" : number.toString()
                anchors.centerIn: parent
            }

        }

    }

    Item {
        focus: true
        anchors.fill: parent
        Keys.onPressed: (event) => {
            switch (event.key) {
            case Qt.Key_Left:
                {
                    app.move_d(3);
                    break;
                };
            case Qt.Key_Right:
                {
                    app.move_d(4);
                    break;
                };
            case Qt.Key_Up:
                {
                    app.move_d(1);
                    break;
                };
            case Qt.Key_Down:
                {
                    app.move_d(2);
                    break;
                };
            default:

                    break;
;
            }
        }
    }

    Text {
        x: 450
        y: 60
        property int score:0
        text: "得分:"+score.toString()
        color: "#95e1d3"
        font.pixelSize: 40
        Component.onCompleted: {
            app.game_changed.connect(function (){score=app.score});
        }
    }

}
