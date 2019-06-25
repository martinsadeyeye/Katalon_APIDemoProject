<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Getting all the Biller on Test Environment</description>
   <name>GetBiller</name>
   <tag></tag>
   <elementGuidId>8fe4d9a8-6a9f-457e-bf45-d6b45d3fa6dc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>publicKey</name>
      <type>Main</type>
      <value>QzAwMDAxMTU0MDF8MTUwOTM3NzUwMjMzNXw2MGFmMDZjYTk4ZWYwNzgyMjIzMDQ5MTY4MmZhMWYwODFlMTAwODg3NDczMzRkYjFjNWQ5MGMzZmM5ZDQwNDEyMmQ1ZThhZjAwM2YyMmU5ZDA1ZjZkM2QyNTg3OWYyZDFhMDRlYjE4NDM3MjVhODYwOGYxMjdhYmJmNzRkYmQwMA==</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>transactionId</name>
      <type>Main</type>
      <value>23</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.9.200.209:6200/microservice/remita/gateway/send/api/bgatesvc/billing/billers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getResponseText()).contains('C0000100600')

WS.verifyElementPropertyValue(response, 'responseData[0].id', 'DEBOLTD')
WS.verifyElementPropertyValue(response, 'responseData[1].id', 'DICE')
WS.verifyElementPropertyValue(response, 'responseData[2].id', 'C0000101701')
WS.verifyElementPropertyValue(response, 'responseData[3].id', 'TECHHEAD')
WS.verifyElementPropertyValue(response, 'responseData[4].id', 'SHELL')
WS.verifyElementPropertyValue(response, 'responseData[5].id', 'MAVETECH')
WS.verifyElementPropertyValue(response, 'responseData[6].id', 'VICKY')
WS.verifyElementPropertyValue(response, 'responseData[7].id', 'WALMART')
WS.verifyElementPropertyValue(response, 'responseData[8].id', 'WARTECH')
WS.verifyElementPropertyValue(response, 'responseData[9].id', 'C0000100600')


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
