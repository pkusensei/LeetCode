using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsValidSudoku(char[][] board)
    {
        const int N = 9;
        int mask = 0;
        foreach (var row in board)
        {
            mask = 0;
            foreach (var num in row)
            {
                if (!Check(ref mask, num)) { return false; }
            }
        }
        for (int c = 0; c < N; c++)
        {
            mask = 0;
            for (int r = 0; r < N; r++)
            {
                char num = board[r][c];
                if (!Check(ref mask, num)) { return false; }
            }
        }
        for (int r = 0; r < N; r += 3)
        {
            for (int c = 0; c < N; c += 3)
            {
                if (!CheckBox(r, c)) { return false; }
            }
        }
        return true;

        bool CheckBox(int row, int col)
        {
            int mask = 0; ;
            for (int r = row; r < row + 3; r++)
            {
                for (int c = col; c < col + 3; c++)
                {
                    char num = board[r][c];
                    if (!Check(ref mask, num)) { return false; }
                }
            }
            return true;
        }

        static bool Check(ref int mask, char num)
        {
            if (num is >= '1' and <= '9')
            {
                int bit = num - '1';
                if (((mask >> bit) & 1) == 1) { return false; }
                mask |= 1 << bit;
            }
            return true;
        }
    }
}