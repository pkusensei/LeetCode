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
        var a = TreeNode.Make([1, 2, 3, 4]);
        var b = sol.Tree2str(a);
        var c = "1(2(4))(3)";
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 3, null, 4]);
        var b = sol.Tree2str(a);
        var c = "1(2()(4))(3)";
        Assert.AreEqual(c, b);
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