using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<string> FindAllRecipes(string[] recipes, IList<IList<string>> ingredients, string[] supplies)
    {
        Dictionary<string, List<string>> adj = [];
        Dictionary<string, int> indegs = [];
        foreach (var item in recipes.Zip(ingredients))
        {
            (var recipe, var ingreds) = item;
            foreach (var ingred in ingreds)
            {
                if (!adj.TryAdd(ingred, [recipe])) { adj[ingred].Add(recipe); }
                if (!indegs.TryAdd(recipe, 1)) { indegs[recipe] += 1; }
            }
        }
        Queue<string> queue = new(supplies);
        List<string> res = [];
        while (queue.TryDequeue(out var node))
        {
            if (recipes.Contains(node)) { res.Add(node); }
            if (adj.TryGetValue(node, out var children))
            {
                foreach (var item in children)
                {
                    if (indegs.TryGetValue(item, out var deg))
                    {
                        indegs[item] -= 1;
                        if (deg == 1) { queue.Enqueue(item); }
                    }
                }
            }
        }
        return res;
    }
}
