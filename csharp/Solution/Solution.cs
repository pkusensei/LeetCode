using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool CanArrange(int[] arr, int k)
    {
        var rems = new int[k];
        foreach (var item in arr)
        {
            rems[(item % k + k) % k] += 1;
        }
        if ((rems[0] & 1) == 1) { return false; }
        for (int i = 1; i < k; i++)
        {
            if (rems[i] != rems[k - i]) { return false; }
            if (i == k - i && (rems[i] & 1) == 1) { return false; }
        }
        return true;
    }
}
