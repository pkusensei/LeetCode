using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> RemoveAnagrams(string[] words)
    {
        List<string> res = [];
        for (int left = 0; left < words.Length;)
        {
            res.Add(words[left]);
            int[] f1 = Count(words[left]);
            int right = left;
            while (right < words.Length && f1.SequenceEqual(Count(words[right])))
            {
                right += 1;
            }
            left = right;
        }
        return res;

        static int[] Count(ReadOnlySpan<char> s)
        {
            int[] f = new int[26];
            foreach (var c in s)
            {
                f[c - 'a'] += 1;
            }
            return f;
        }
    }
}
