using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string FindLexSmallestString(string s, int a, int b)
    {
        int n = s.Length;
        Queue<char[]> queue = [];
        queue.Enqueue(s.ToCharArray());
        HashSet<string> seen = [s];
        while (queue.TryDequeue(out var curr))
        {
            char[] temp1 = [.. curr];
            for (int i = 1; i < n; i += 2)
            {
                temp1[i] = (char)(temp1[i] + a);
                if (temp1[i] > '9') { temp1[i] = (char)(temp1[i] - 10); }
            }
            if (seen.Add(new(temp1))) { queue.Enqueue(temp1); }
            char[] temp2 = new char[n];
            Array.Copy(curr, b, temp2, 0, n - b);
            Array.Copy(curr, 0, temp2, n - b, b);
            if (seen.Add(new(temp2))) { queue.Enqueue(temp2); }
        }
        return seen.Min();
    }
}
