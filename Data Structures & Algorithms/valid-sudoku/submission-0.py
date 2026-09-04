class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        NUMS = set(str(i) for i in range(1, 10))
        def boardMask(board, x, y):
            return [
                board[x - 1][y - 1], board[x][y - 1], board[x + 1][y - 1],
                board[x - 1][y    ], board[x][y    ], board[x + 1][y    ],
                board[x - 1][y + 1], board[x][y + 1], board[x + 1][y + 1]
            ]
        def checkArr(arr):
            nums = set(str(i) for i in range(1, 10))
            for i in arr:
                if i == ".":
                    continue
                nums.remove(i)
        try:
            for row in board:
                checkArr(row)
            for col in [list(c) for c in zip(*board)]:
                checkArr(col)
            for (x, y) in [
                (1, 1), (4, 1), (7, 1),
                (1, 4), (4, 4), (7, 4),
                (1, 7), (4, 7), (7, 7)]:
                checkArr(boardMask(board, x, y))
        except KeyError:
            return False
        return True
