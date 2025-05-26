using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("123", 2)]
    [DataRow("112", 1)]
    [DataRow("12345", 0)]
    public void TestMethod1(string s, int exp)
    {
        Assert.AreEqual(exp, sol.CountBalancedPermutations(s));
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