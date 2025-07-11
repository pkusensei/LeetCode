using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string SimplifyPath(string path)
    {
        Stack<string> st = [];
        foreach (var item in path.Split('/', StringSplitOptions.RemoveEmptyEntries))
        {
            switch (item)
            {
                case ".": break;
                case "..": st.TryPop(out _); break;
                default: st.Push(item); break;
            }
        }
        return $"/{string.Join('/', st.Reverse())}";
    }
}