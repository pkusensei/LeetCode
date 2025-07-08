using System.Reflection.Metadata;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void SolveSudoku(char[][] board)
    {
        Backtrack();

        bool Backtrack()
        {
            for (int r = 0; r < 9; r++)
            {
                for (int c = 0; c < 9; c++)
                {
                    if (board[r][c] == '.')
                    {
                        for (int ch = '1'; ch <= '9'; ch += 1)
                        {
                            if (Check(r, c, (char)ch))
                            {
                                board[r][c] = (char)ch;
                                if (Backtrack()) { return true; }
                                else { board[r][c] = '.'; }
                            }
                        }
                        return false;
                    }
                }
            }
            return true;
        }

        bool Check(int r, int c, char ch)
        {
            for (int rr = 0; rr < 9; rr++)
            {
                if (board[rr][c] == ch) { return false; }
            }
            for (int cc = 0; cc < 9; cc++)
            {
                if (board[r][cc] == ch) { return false; }
            }
            int start_r = r / 3 * 3;
            int start_c = c / 3 * 3;
            for (int dr = 0; dr < 3; dr++)
            {
                for (int dc = 0; dc < 3; dc++)
                {
                    if (board[start_r + dr][start_c + dc] == ch) { return false; }
                }
            }
            return true;
        }
    }
}
