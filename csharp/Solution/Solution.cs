using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> PancakeSort(int[] arr)
    {
        int n = arr.Length;
        List<int> res = [];
        for (int right = n - 1; right >= 0; right -= 1)
        {
            int max = arr[right];
            int max_i = right;
            for (int i = right - 1; i >= 0; i -= 1)
            {
                if (arr[i] > max)
                {
                    max = arr[i];
                    max_i = i;
                }
            }
            if (max_i == right) { continue; }
            else if (max_i > 0)
            {
                res.Add(1 + max_i);
                Array.Reverse(arr, 0, 1 + max_i);
            }
            res.Add(1 + right);
            Array.Reverse(arr, 0, 1 + right);
        }
        return res;
    }
}
