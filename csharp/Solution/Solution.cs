using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CompareVersion(string version1, string version2)
    {
        int[] s1 = [.. version1.Split('.').Select(int.Parse)];
        int[] s2 = [.. version2.Split('.').Select(int.Parse)];
        int i1 = 0;
        int i2 = 0;
        for (; i1 < s1.Length || i2 < s2.Length; i1 += 1, i2 += 1)
        {
            if (i1 < s1.Length && i2 < s2.Length)
            {
                if (s1[i1] == s2[i2]) { continue; }
                return s1[i1] < s2[i2] ? -1 : 1;
            }
            if (i1 >= s1.Length && s2[i2] > 0) { return -1; }
            if (i2 >= s2.Length && s1[i1] > 0) { return 1; }
        }
        return 0;
    }
}