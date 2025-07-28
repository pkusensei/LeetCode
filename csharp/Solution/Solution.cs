using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> CountSmaller(int[] nums)
    {
        const int DIFF = 10_000;
        int[] res = new int[nums.Length];
        BIT tree = new(1 + 2 * DIFF);
        for (int i = nums.Length - 1; i >= 0; i -= 1)
        {
            int val = nums[i] + DIFF;
            res[i] = tree.Query(val - 1);
            tree.Insert(val, 1);
        }
        return res;
    }
}

internal class BIT
{
    public BIT(int n)
    {
        Tree = new long[1 + n];
        N = n;
    }

    public long[] Tree { get; }
    public int N { get; }

    public void Insert(int i, int val)
    {
        while (i <= N)
        {
            Tree[i] += val;
            i += i & -i;
        }
    }

    public int Query(int i)
    {
        long res = 0;
        while (i > 0)
        {
            res += Tree[i];
            i -= i & -i;
        }
        return (int)res;
    }
}