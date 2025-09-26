using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] ConstructArray(int n, int k)
    {
        int[] res = [.. Enumerable.Range(1, n)];
        for (int i = 1; i <= k; i++)
        {
            Array.Reverse(res, i, n - i);
        }
        return res;
    }

    public int[] WithTwoPtrs(int n, int k)
    {
        List<int> res = new(n);
        int left = 1;
        int right = n;
        while (left <= right)
        {
            if ((k & 1) == 1)
            {
                res.Add(left);
                left += 1;
            }
            else
            {
                res.Add(right);
                right -= 1;
            }
            if (k > 1) { k -= 1; }
        }
        return [.. res];
    }
}