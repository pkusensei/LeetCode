using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximalRectangle(char[][] matrix)
    {
        int cols = matrix[0].Length;
        int[] arr = new int[1 + cols];
        int res = 0;
        foreach (var line in matrix)
        {
            for (int i = 0; i < cols; i++)
            {
                if (line[i] == '1') { arr[i] += 1; }
                else { arr[i] = 0; }
            }
            Stack<int> st = new(1 + cols);
            for (int i = 0; i <= cols; i++)
            {
                while (st.TryPop(out int top))
                {
                    if (arr[top] > arr[i]) // mono increasing stack
                    {
                        if (!st.TryPeek(out int prev)) { prev = -1; }
                        res = int.Max(res, (i - prev - 1) * arr[top]);
                    }
                    else
                    {
                        st.Push(top);
                        break;
                    }
                }
                st.Push(i);
            }
        }
        return res;
    }
}