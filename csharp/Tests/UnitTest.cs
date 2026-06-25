using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = TreeNode.Make([5, 4, 8, 11, null, 17, 4, 7, 1, null, null, 5, 3]);
        var b = sol.SufficientSubset(a, 22).ToString();
        Assert.AreEqual("[5,4,8,11,null,17,4,7,null,null,null,5]", b);
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}