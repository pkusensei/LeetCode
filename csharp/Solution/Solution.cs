using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int WithMonoStack(int[] arr)
    {
        Stack<int> st = [];
        st.Push(int.MaxValue);
        int res = 0;
        foreach (var right in arr)
        {
            while (st.TryPeek(out int mid) && mid <= right)
            {
                st.Pop(); // Pop `mid` and calculate cost
                int val = st.TryPeek(out int left) ? int.Min(left, right) : right;
                res += mid * val;
            }
            st.Push(right);
        }
        while (st.Count > 2)
        {
            res += st.Pop() * st.Peek();
        }
        return res;
    }

    public int MctFromLeafValues(int[] arr)
    {
        int n = arr.Length;
        long[,] dp = new long[n, n];
        long[,] max = new long[n, n];
        for (int left = 0; left < n; left++)
        {
            max[left, left] = arr[left];
            for (int right = 1 + left; right < n; right++)
            {
                max[left, right] = long.Max(max[left, right - 1], arr[right]);
                dp[left, right] = long.MaxValue >> 1;
            }
        }
        for (int right = 0; right < n; right++)
        {
            for (int left = right - 1; left >= 0; left -= 1)
            {
                for (int mid = left; mid < right; mid++)
                {
                    dp[left, right] = long.Min(dp[left, right],
                        dp[left, mid] + dp[1 + mid, right] + max[left, mid] * max[1 + mid, right]);
                }
            }
        }
        for (int left = n - 1; left >= 0; left -= 1)
        {
            for (int right = left; right < n; right++)
            {
                for (int mid = left; mid < right; mid++)
                {
                    dp[left, right] = long.Min(dp[left, right],
                        dp[left, mid] + dp[1 + mid, right] + max[left, mid] * max[1 + mid, right]);
                }
            }
        }
        return (int)dp[0, n - 1];

        (long val, int max) Dfs(int left, int right)
        {
            if (left == right) { return (0, arr[left]); }
            long res = long.MaxValue;
            int max = arr[left];
            for (int mid = left; mid < right; mid++)
            {
                (var left_val, var left_max) = Dfs(left, mid);
                (var right_val, var right_max) = Dfs(1 + mid, right);
                res = long.Min(res, left_val + right_val + left_max * right_max);
                max = int.Max(max, arr[1 + mid]);
            }
            return (res, max);
        }
    }
}
