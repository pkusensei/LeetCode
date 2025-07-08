using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int StrStr(string haystack, string needle)
    {
        int[] lps = new int[needle.Length];
        int len = 0;
        for (int i = 1; i < needle.Length; i++)
        {
            while (len > 0 && needle[i] != needle[len]) { len = lps[len - 1]; }
            if (needle[i] == needle[len]) { len += 1; }
            lps[i] = len;
        }
        len = 0;
        for (int i = 0; i < haystack.Length; i += 1)
        {
            while (len > 0 && haystack[i] != needle[len]) { len = lps[len - 1]; }
            if (needle[len] == haystack[i]) { len += 1; }
            if (len == needle.Length) { return i - len + 1; }
        }
        return -1;
    }
}
