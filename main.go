package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"log"
	"math/big"
	"os"
	"os/exec"
)

// Configuration and test data from Xlayer
const (
	verifierKeyPath = "verifier-key-fork9.json"

	rollupChainID    uint64 = 196
	rollupForkID     uint64 = 9
	initNumBatch     uint64 = 110305
	finalNewBatch    uint64 = 110317
	newLocalExitRoot        = "0xd94205b12f066e9cd161126d6dd02ce3fbcd624f1bca4701505d43534bfce098"
	oldStateRoot            = "0x6a8dc4703a0172a13404a24c68c52c07be64735bce138469269f9dc3247822d8" // corresponding to initNumBatch
	newStateRoot            = "0x5d2d62ab2710a77433cba4acee22c106c58306944d83d7d6c7f0a76344773225" // corresponding to finalNewBatch
	oldAccInputHash         = "0x335db443dbc2f5f37897994c375031f6c34b22429dd1ecd6a637321f2c18b3f6" // corresponding to initNumBatch
	newAccInputHash         = "0x77d439fd4a82fc32a1ce48e91667671960beb9d7ffe6b79fac22f7e084a73e5b" // corresponding to finalNewBatch
	beneficiary             = "0x20A53dCb196cD2bcc14Ece01F358f1C849aA51dE"
	proof                   = "1397200f5b75995df579aea5002af81e7950c577b2870c4fb410d12e749b91ae03c490cd7787ac1ab00d05fa3f99887e3f0b6ec87fbe4fe9ab554fa26e7f74c716fcfd016a8b4750ce1f2a2fc9dd24ecf0986317d24d49823f2d85a77e382cf01aa4d570d51613c1e563dfd6cc14e171d500ac8d66b5c21bdfbf844108b08b86002c65c475db3b09c70283b704be4b4adc3e12e4fd3fec5d5df2ad0af87813f40e941f5aef70b6d828430af4437b69b84db735e94cb8a6b5f2c6b892397213980c83904751d57a214008ec39b14b1f01daa5843662a7df42491a8b5417a57f300a0d8147cfc596cb1ddaa1da7c9b9c71846d8fe649c7865c80c7d9915e2bbbc719a77f52462f156ee4c39983bc751a010ef0147d7d161f7aa34043845d65f9c22923f0f17d03786cf9be2856e0d73bc48768a8718f2ac50a26cc85c9114f297123610b97cf9036c228afabce0f5facbe36abe383f92f8a763f21eb7ad30bd6ce057ded654f08c9f2fe3a6836eca8a6fd9c174e2e5fee9ae0ccb9957cd1d23eb714b2e9f9fea41091383f3641319b40fcc6098dabc17b609c26d4ae72664d33cd0379cdca3f67a84976b4aa4fe24154ffb71a421350f7a0cd48557b57b13819510565159a44cf1f2031bf3fb075e0c00f96a894c6b833645d5374abde52d67ec912834ada8c62594b15f6ef0945db2b83785f0ea6ab0474ec819ab762307528ac20c6c6f94d7ad9e1a9f716080a28771106bfaa89204bafc637d703e04bee0cfe044aac50b932069d3fa01142e1b2e5f4a20639930d62f9de98e348f14a67cdcb27efe2aaea7d46be036293567a44d5b0ca6098dd16dbd6840e0a65f5e29e66b72cc3e628572b4c679b332f4c86fb356152f31f1cbcc5481be23873bdeaeed0bd13abbe95ff2265631ec0571e407f2fe6f81bfaacc552d3ed264605be756d9a60184b4a3853c660dc5dfbdb98d0aee39ca49a24504caff890c6568ec4c989137e1c6fec8bc82f0d47d6b6cc765c1e989f451c14492d453b3c3e918bef7db1ed5f1e570182860f4fcced583a4e42319f3fcdaee41cdfffd20c3cd9ce1f5cba88c1"
)

const rFieldNumericStr = "21888242871839275222246405745257275088548364400416034343698204186575808495617"

func abiEncodePacked(args ...interface{}) ([]byte, error) {
	var buffer bytes.Buffer

	for _, arg := range args {
		switch v := arg.(type) {
		case []byte:
			buffer.Write(v)
		case string:
			bytes, err := decodeHexString(v)
			if err != nil {
				return nil, err
			}
			buffer.Write(bytes)
		case uint64:
			var bytes [8]byte
			binary.BigEndian.PutUint64(bytes[:], v)
			buffer.Write(bytes[:])
		default:
			return nil, fmt.Errorf("unsupported arg type: %T", arg)
		}
	}

	return buffer.Bytes(), nil
}

func decodeHexString(s string) ([]byte, error) {
	if len(s) > 2 && (s[:2] == "0x" || s[:2] == "0X") {
		return hex.DecodeString(s[2:])
	}
	return nil, fmt.Errorf("string should start with '0x'")
}

func generatePubInput() (*big.Int, error) {
	msgSender, err := decodeHexString(beneficiary)
	if err != nil {
		return nil, fmt.Errorf("error converting address: %v", err)
	}

	inputData, err := abiEncodePacked(
		msgSender,
		oldStateRoot,
		oldAccInputHash,
		initNumBatch,
		rollupChainID,
		rollupForkID,
		newStateRoot,
		newAccInputHash,
		newLocalExitRoot,
		finalNewBatch,
	)
	if err != nil {
		return nil, fmt.Errorf("error encoding data: %v", err)
	}

	// fmt.Println("Input snark bytes:", hex.EncodeToString(inputData))

	hash := sha256.Sum256(inputData)
	hashInt := new(big.Int).SetBytes(hash[:])

	rField := new(big.Int)
	rField.SetString(rFieldNumericStr, 10)
	pubs := new(big.Int).Mod(hashInt, rField)

	// fmt.Println("Pub signal:", fmt.Sprintf("0x%x", pubs))

	return pubs, nil
}

func build() error {
	cmd := exec.Command("cargo", "build", "--release", "--features", "bins")
	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	return cmd.Run()
}

func verify(proof string, pubs *big.Int) error {
	cmd := exec.Command("./target/release/verifier", "--proof-fmt", "hex-string", verifierKeyPath, proof, pubs.String())
	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	return cmd.Run()
}

func main() {
	if err := build(); err != nil {
		log.Fatalf("Build failed: %v", err)
	}

	pubs, err := generatePubInput()
	if err != nil {
		log.Fatalf("Error generating public input: %v", err)
	}

	if err := verify(proof, pubs); err != nil {
		log.Fatalf("Verifier execution failed: %v", err)
	}
}
