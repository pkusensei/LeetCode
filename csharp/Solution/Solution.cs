using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountBattleships(char[][] board)
    {
        int rows = board.Length;
        int cols = board[0].Length;
        int res = 0;
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (board[r][c] == 'X')
                {
                    if (r == 0 && c == 0) { res += 1; }
                    else if (r == 0 && board[r][c - 1] == '.' || c == 0 && board[r - 1][c] == '.') { res += 1; }
                    else if (r > 0 && board[r - 1][c] == '.' && c > 0 && board[r][c - 1] == '.') { res += 1; }
                }
            }
        }
        return res;
    }
}
