using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(5, new[] { 0, 0, 0 }, 'a')]
    [DataRow(10, new[] { 0, 1, 0, 1 }, 'b')]
    public void TestMethod1(int k, int[] ops, char exp)
    {
        Assert.AreEqual(exp, sol.KthCharacter(k, ops));
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