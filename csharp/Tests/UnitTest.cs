using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("ababccc", 5)]
    [DataRow("aba", 2)]
    [DataRow("aa", 1)]
    public void TestMethod1(string s, int exp)
    {
        Assert.AreEqual(exp, sol.MaxUniqueSplit(s));
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