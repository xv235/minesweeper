#include <iostream>
#include <vector>
#include <random>
#include <iomanip>

using namespace std;

class Minesweeper {
private:
    struct Cell {
        bool isMine = false;
        bool isRevealed = false;
        bool isFlagged = false;
        int adjacentMines = 0;
    };

    vector<vector<Cell>> board;
    int width;
    int height;
    int mineCount;
    bool gameOver;
    int revealedCount;

    const int dx[8] = {-1, -1, -1, 0, 0, 1, 1, 1};
    const int dy[8] = {-1, 0, 1, -1, 1, -1, 0, 1};

public:
    Minesweeper(int w, int h, int mines) :
        width(w),
        height(h),
        mineCount(mines),
        gameOver(false),
        revealedCount(0) {
        board.resize(height, vector<Cell>(width));
        initializeBoard();
    }

    void initializeBoard() {
        random_device rd;
        mt19937 gen(rd());
        uniform_int_distribution<> disH(0, height - 1);
        uniform_int_distribution<> disW(0, width - 1);

        int placedMines = 0;
        while (placedMines < mineCount) {
            int x = disW(gen);
            int y = disH(gen);
            if (!board[y][x].isMine) {
                board[y][x].isMine = true;
                placedMines++;
            }
        }

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                if (!board[y][x].isMine) {
                    int count = 0;
                    for (int i = 0; i < 8; i++) {
                        int newY = y + dy[i];
                        int newX = x + dx[i];
                        if (isValid(newX, newY) && board[newY][newX].isMine) {
                            count++;
                        }
                    }
                    board[y][x].adjacentMines = count;
                }
            }
        }
    }

    bool isValid(int x, int y) const {
        return x >= 0 && x < width && y >= 0 && y < height;
    }

    void reveal(int x, int y) {
        if (!isValid(x, y) || board[y][x].isRevealed || board[y][x].isFlagged) {
            return;
        }

        board[y][x].isRevealed = true;
        revealedCount++;

        if (board[y][x].isMine) {
            gameOver = true;
            return;
        }

        if (board[y][x].adjacentMines == 0) {
            for (int i = 0; i < 8; i++) {
                int newX = x + dx[i];
                int newY = y + dy[i];
                if (isValid(newX, newY) && !board[newY][newX].isRevealed) {
                    reveal(newX, newY);
                }
            }
        }
    }

    void toggleFlag(int x, int y) {
        if (!isValid(x, y) || board[y][x].isRevealed) {
            return;
        }
        board[y][x].isFlagged = !board[y][x].isFlagged;
    }

    void display() const {
        cout << "    ";
        for (int x = 0; x < width; x++) {
            cout << setw(2) << x << " ";
        }
        cout << "\n    ";
        for (int x = 0; x < width; x++) {
            cout << "---";
        }
        cout << '\n';

        for (int y = 0; y < height; y++) {
            cout << setw(2) << y << " |";
            for (int x = 0; x < width; x++) {
                const Cell& cell = board[y][x];
                if (cell.isRevealed) {
                    if (cell.isMine) {
                        cout << " * ";
                    } else {
                        cout << " " << (cell.adjacentMines ? to_string(cell.adjacentMines) : " ") << " ";
                    }
                } else if (cell.isFlagged) {
                    cout << " F ";
                } else {
                    cout << " # ";
                }
            }
            cout << '\n';
        }
    }

    bool isGameOver() const { return gameOver; }

    bool isVictory() const {
        return revealedCount == (width * height - mineCount);
    }
};

int main() {
    const int WIDTH = 9;
    const int HEIGHT = 9;
    const int MINES = 10;

    cout << "Welcome to Minesweeper, " << "xv235" << "!\n";
    cout << "Current Date: 2025-02-23\n";
    cout << "Current Time: 09:07:53 UTC\n";
    cout << "Commands:\n";
    cout << "r x y - Reveal cell at (x,y)\n";
    cout << "f x y - Toggle flag at (x,y)\n";
    cout << "q - Quit game\n\n";

    Minesweeper game(WIDTH, HEIGHT, MINES);

    while (true) {
        game.display();

        if (game.isGameOver()) {
            cout << "\nGame Over! You hit a mine!\n";
            break;
        }

        if (game.isVictory()) {
            cout << "\nCongratulations! You won!\n";
            break;
        }

        cout << "\nEnter command: ";
        char command;
        int x, y;

        cin >> command;
        if (command == 'q') break;

        if (command != 'r' && command != 'f') {
            cout << "Invalid command!\n";
            cin.clear();
            cin.ignore(10000, '\n');
            continue;
        }

        if (!(cin >> x >> y)) {
            cout << "Invalid coordinates!\n";
            cin.clear();
            cin.ignore(10000, '\n');
            continue;
        }

        if (command == 'r') {
            game.reveal(x, y);
        } else if (command == 'f') {
            game.toggleFlag(x, y);
        }
    }

    return 0;
}