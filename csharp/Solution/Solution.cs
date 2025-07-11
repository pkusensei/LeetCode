using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LargestRectangleArea(int[] heights)
    {
        int n = heights.Length;
        int[] right_smaller = new int[n];
        Array.Fill(right_smaller, n);
        Stack<int> st = [];
        for (int i = 0; i < n; i++)
        {
            while (st.TryPeek(out int top) && heights[top] > heights[i])
            {
                st.Pop();
                right_smaller[top] = i;
            }
            st.Push(i);
        }
        st.Clear();
        int[] left_smaller = new int[n];
        Array.Fill(left_smaller, -1);
        for (int i = n - 1; i >= 0; i -= 1)
        {
            while (st.TryPeek(out int top) && heights[top] > heights[i])
            {
                st.Pop();
                left_smaller[top] = i;
            }
            st.Push(i);
        }
        int res = 0;
        foreach (var ((h, right), left) in heights.Zip(right_smaller).Zip(left_smaller))
        {
            res = int.Max(res, h * (right - left - 1));
        }
        return res;
    }
}