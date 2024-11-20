using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    (int exp, int act) Run(IList<int?> nums, int c)
    {
        var a = TreeNode.Make(nums);
        var b = sol.MaxAncestorDiff(a);
        return (c, b);
    }

    [TestMethod]
    public void TestMethod1()
    {
        var t = Run([8, 3, 10, 1, 6, null, 14, null, null, 4, 7, 13], 7);
        Assert.AreEqual(t.exp, t.act);
    }


    [TestMethod]
    public void TestMethod2()
    {
        var t = Run([1, null, 2, null, 0, 3], 3);
        Assert.AreEqual(t.exp, t.act);
    }

    [TestMethod]
    public void TestMethod3()
    {
    }
}