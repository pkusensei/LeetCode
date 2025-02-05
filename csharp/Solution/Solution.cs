using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] MinOperations(string boxes)
    {
        var res = new int[boxes.Length];
        var balls = 0;
        var moves = 0;
        foreach (var (i, b) in boxes.Select((v, i) => (i, v)))
        {
            moves += balls;
            res[i] += moves;
            balls += b == '1' ? 1 : 0;
        }
        balls = 0;
        moves = 0;
        foreach (var (i, b) in boxes.Select((v, i) => (i, v)).Reverse())
        {
            moves += balls;
            res[i] += moves;
            balls += b == '1' ? 1 : 0;
        }
        return res;
    }
}
