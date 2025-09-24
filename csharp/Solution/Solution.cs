using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FindClosestElements(int[] arr, int k, int x)
    {
        int left = 0;
        int right = arr.Length - 1;
        while (right - left + 1 > k)
        {
            if (int.Abs(arr[right] - x) >= int.Abs(arr[left] - x)) { right -= 1; }
            else { left += 1; }
        }
        return arr[left..(1 + right)];
    }
}