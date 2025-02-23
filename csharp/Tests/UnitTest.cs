using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 4, 5, 3, 6, 7 }, new[] { 4, 5, 2, 6, 7, 3, 1 }, "[1,2,3,4,5,6,7]")]
    [DataRow(new[] { 2, 1, 3 }, new[] { 3, 1, 2 }, "[2,null,1,null,3]")]
    public void TestMethod1(int[] pre, int[] post, string exp)
    {
        Assert.AreEqual(exp, sol.ConstructFromPrePost(pre, post).ToString());
        Assert.AreEqual(exp, sol.TwoPtrs(pre, post).ToString());
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