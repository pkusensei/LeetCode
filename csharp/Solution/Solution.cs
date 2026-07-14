using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] RelativeSortArray(int[] arr1, int[] arr2)
    {
        Dictionary<int, int> dict = [];
        for (int i = 0; i < arr2.Length; i++)
        {
            dict[arr2[i]] = i;
        }
        Array.Sort(arr1, (a, b) =>
        {
            int aa = dict.GetValueOrDefault(a, a);
            int bb = dict.GetValueOrDefault(b, b);
            return aa.CompareTo(bb);
        });
        return arr1;
    }
}
