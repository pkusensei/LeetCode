using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximumSum(int[] arr)
    {
        int no_delete = 0;
        int with_delete = 0;
        int res = int.MinValue;
        bool started = false;
        foreach (var item in arr)
        {
            if (started) { with_delete = int.Max(item + with_delete, no_delete); }
            no_delete = int.Max(item, item + no_delete);
            int curr = started ? int.Max(no_delete, with_delete) : no_delete;
            res = int.Max(res, curr);
            started = true;
        }
        return res;
    }
}

