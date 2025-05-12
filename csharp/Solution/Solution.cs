using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] FindEvenNumbers(int[] digits)
    {
        Span<int> freq = stackalloc int[10];
        foreach (var d in digits)
        {
            freq[d] += 1;
        }
        Span<int> curr = stackalloc int[10];
        List<int> res = [];
        for (int num = 100; num < 1000; num += 2)
        {
            int val = num;
            freq.CopyTo(curr);
            bool valid = true;
            while (val > 0)
            {
                int d = val % 10;
                val /= 10;
                curr[d] -= 1;
                if (curr[d] < 0)
                {
                    valid = false;
                    break;
                }
            }
            if (valid) { res.Add(num); }
        }
        return res.ToArray();
    }
}
