using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<bool> CheckIfPrerequisite(int numCourses, int[][] prerequisites, int[][] queries)
    {
        bool[,] pre = new bool[numCourses, numCourses];
        foreach (var item in prerequisites)
        {
            pre[item[0], item[1]] = true;
        }
        // Why this loop sequence makes a difference
        for (int mid = 0; mid < numCourses; mid++)
        {
            for (int src = 0; src < numCourses; src++)
            {
                for (int tgt = 0; tgt < numCourses; tgt++)
                {
                    pre[src, tgt] = pre[src, tgt] || (pre[src, mid] && pre[mid, tgt]);
                }
            }
        }
        return [.. queries.Select(q => pre[q[0], q[1]])];
    }

    public IList<bool> WithTopoSort(int numCourses, int[][] prerequisites, int[][] queries)
    {
        var indegs = new int[numCourses];
        var adj = Enumerable.Range(0, numCourses).Select(_ => new List<int>()).ToList();
        foreach (var edge in prerequisites)
        {
            adj[edge[0]].Add(edge[1]);
            indegs[edge[1]] += 1;
        }
        var queue = new Queue<int>();
        for (int i = 0; i < numCourses; i++)
        {
            if (indegs[i] == 0) { queue.Enqueue(i); }
        }
        var all_preqs = Enumerable.Range(0, numCourses).Select(_ => new HashSet<int>()).ToList();
        while (queue.TryDequeue(out var curr))
        {
            foreach (var next in adj[curr])
            {
                all_preqs[next].Add(curr);
                foreach (var preq in all_preqs[curr])
                {
                    all_preqs[next].Add(preq);
                }
                indegs[next] -= 1;
                if (indegs[next] == 0) { queue.Enqueue(next); }
            }
        }
        return [.. queries.Select(q => all_preqs[q[1]].Contains(q[0]))];
    }
}
