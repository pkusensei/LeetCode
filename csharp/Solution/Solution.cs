using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string SolveEquation(string equation)
    {
        int xfactor = 0;
        int num = 0;
        int sign = 1;
        int is_left = 1;
        int curr_num = 0;
        for (int i = 0; i < equation.Length; i++)
        {
            switch (equation[i])
            {
                case 'x':
                    if (i > 0 && equation[i - 1] == '0') { break; }
                    if (curr_num == 0)
                    {
                        xfactor += sign;
                    }
                    else
                    {
                        xfactor += sign * curr_num;
                        curr_num = 0;
                        sign = is_left;
                    }
                    break;
                case '+':
                    num += sign * curr_num;
                    sign = is_left;
                    curr_num = 0;
                    break;
                case '-':
                    num += sign * curr_num;
                    sign = -1 * is_left;
                    curr_num = 0;
                    break;
                case '=':
                    num += sign * curr_num;
                    is_left = -1;
                    sign = is_left;
                    curr_num = 0;
                    break;
                default:
                    curr_num = 10 * curr_num + equation[i] - '0';
                    break;
            }
        }
        num += sign * curr_num;
        return (xfactor == 0, num == 0) switch
        {
            (true, true) => "Infinite solutions",
            (true, false) => "No solution",
            _ => $"x={-num / xfactor}"
        };
    }
}
